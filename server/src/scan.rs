use std::path::PathBuf;

use chrono::Datelike;
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use dlsite::product::Product;
use futures::stream::StreamExt;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{debug, error, info, warn};
use walkdir::WalkDir;

use crate::cornucopia::queries::{
    circle::{upsert_circle, UpsertCircleParams},
    genre::{
        insert_product_genre, upsert_genre, upsert_product_usergenre, InsertProductGenreParams,
        UpsertGenreParams, UpsertProductUsergenreParams,
    },
    product::{
        delete_product, delete_product_genre, delete_product_usergenre, product_ids,
        upsert_product, UpsertProductParams,
    },
};

static DLSITE_FOLDER_REGEX: Lazy<Regex> = Lazy::new(|| regex::Regex::new(r"(?i)RJ\d+").unwrap());

/// Scan folders for RJ folders and fetch metadata for each RJ folder.
///
/// # Arguments
/// * `folders` - List of folders to scan
/// * `force` - Force fetch metadata for each RJ folder even if the metadata already exists in db.
pub async fn scan(folders: &Vec<PathBuf>, force: bool, pool: &Pool) -> anyhow::Result<()> {
    info!("Starting scan");
    if force {
        info!("Force scan enabled. Data will be overwritten.");
    }

    let client = pool.get().await?;

    let local_available_id_and_paths = scan_rj_folder(folders).await;

    let local_available_ids = local_available_id_and_paths
        .iter()
        .map(|(id, _)| id.to_string())
        .collect::<Vec<_>>();
    let db_available_ids = product_ids().bind(&client).all().await?;
    debug!("{} products already in db", db_available_ids.len());

    let ids_to_fetch = if force {
        local_available_id_and_paths
    } else {
        local_available_id_and_paths
            .into_iter()
            .filter(|(id, _)| {
                !db_available_ids
                    .iter()
                    .any(|db_id| db_id.to_uppercase() == id.to_uppercase())
            })
            .collect::<Vec<_>>()
    };

    let dlsite_client = dlsite::DlsiteClient::default();
    let fetch_tasks = ids_to_fetch.into_iter().map(|data| {
        let dlsite_client = dlsite_client.clone();
        let id = data.0.clone();

        async move {
            tokio::spawn(async move {
                let res = dlsite_client.get_product(&id).await;
                if let Err(err) = &res {
                    error!("Failed to fetch metadata for {}: {}", id, err);
                }
                (res, data.1.clone())
            })
            .await
        }
    });
    info!(
        "{} new product(s) detected. Fetching metadata.",
        fetch_tasks.len()
    );
    let mut dlsite_fetch_result = futures::stream::iter(fetch_tasks)
        .buffer_unordered(20)
        .collect::<Vec<_>>()
        .await;
    let metadata = dlsite_fetch_result
        .drain(..)
        .filter_map(|res| res.ok())
        .filter_map(|res| res.0.ok().map(|data| (data, res.1)))
        .collect::<Vec<_>>();

    debug!("Fetched metadata for {} RJ folders", metadata.len());

    let db_product_update_result = futures::future::join_all(
        metadata
            .into_iter()
            .map(|(product, path)| async move { create_product(pool, product, path).await }),
    )
    .await;

    let succeed_task_count = db_product_update_result
        .iter()
        .filter(|res| {
            if let Err(err) = res {
                error!("Failed to insert update product: {}", err);
                false
            } else {
                true
            }
        })
        .count();

    info!("{} products updated to db", succeed_task_count);

    let product_ids_to_delete = db_available_ids
        .into_iter()
        .filter(|db_id| {
            !local_available_ids
                .iter()
                .any(|local_id| local_id.to_uppercase() == db_id.to_uppercase())
        })
        .collect::<Vec<_>>();

    delete_product_and_relations(pool, &product_ids_to_delete)
        .await
        .map_err(|e| {
            error!("Failed to delete products and relations: {}", e);
            e
        })?;

    info!("Deleted {} products from db", product_ids_to_delete.len());
    info!("Scan finished");

    Ok(())
}

async fn scan_rj_folder(paths: &Vec<PathBuf>) -> Vec<(String, PathBuf)> {
    let mut id_paths = vec![];

    for path in paths {
        let mut it = WalkDir::new(path).into_iter();
        loop {
            let entry = match it.next() {
                None => break,
                Some(Err(err)) => {
                    warn!("Failed to scan folder: {}", err);
                    continue;
                }
                Some(Ok(entry)) => entry,
            };
            if entry.file_type().is_dir() {
                let name = entry.file_name().to_string_lossy();

                if let Some(id) = DLSITE_FOLDER_REGEX.find(&name) {
                    let id = id.as_str().to_uppercase();
                    id_paths.push((id, entry.into_path()));

                    it.skip_current_dir();
                }
            }
        }
    }

    info!("Found {} RJ folders", id_paths.len());

    id_paths
}

async fn create_product(pool: &Pool, product: Product, path: PathBuf) -> Result<(), anyhow::Error> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    upsert_circle()
        .params(
            &transaction,
            &UpsertCircleParams {
                id: product.circle.id.clone(),
                name: product.circle.name,
            },
        )
        .await?;

    upsert_product()
        .params(
            &transaction,
            &UpsertProductParams {
                id: product.id.clone(),
                name: product.title,
                description: None::<&str>,
                series: product.series,
                circle_id: product.circle.id,
                remote_image: product
                    .images
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>(),
                actor: product.people.voice_actor.unwrap_or_default(),
                author: product.people.author.unwrap_or_default(),
                illustrator: product.people.illustrator.unwrap_or_default(),
                price: product.price,
                sale_count: product.sale_count,
                age: product.age_rating.into(),
                // convert chrono date to "time" crate date
                released_at: time::Date::from_calendar_date(
                    product.released_at.year(),
                    time::Month::try_from(u8::try_from(product.released_at.month()).unwrap())
                        .unwrap(),
                    product.released_at.day().try_into().unwrap(),
                )
                .unwrap(),
                rating: product.rating,
                rating_count: product.rate_count.unwrap_or(0),
                comment_count: product.review_count.unwrap_or(0),
                path: path.to_string_lossy(),
            },
        )
        .await?;

    for genre in product.genre {
        upsert_genre()
            .params(
                &transaction,
                &UpsertGenreParams {
                    id: genre.id.clone(),
                    name: genre.name.clone(),
                },
            )
            .await?;
        insert_product_genre()
            .params(
                &transaction,
                &InsertProductGenreParams {
                    product_id: product.id.clone(),
                    genre_id: genre.id,
                },
            )
            .await?;
    }

    for (genre, count) in product.reviewer_genre {
        upsert_genre()
            .params(
                &transaction,
                &UpsertGenreParams {
                    id: genre.id.clone(),
                    name: genre.name.clone(),
                },
            )
            .await?;
        upsert_product_usergenre()
            .params(
                &transaction,
                &UpsertProductUsergenreParams {
                    product_id: product.id.clone(),
                    genre_id: genre.id.clone(),
                    count,
                },
            )
            .await?;
    }

    transaction.commit().await.map_err(|e| {
        error!("Could not commit transaction: {}", e);
        e
    })?;

    Ok(())
}

async fn delete_product_and_relations(pool: &Pool, ids: &Vec<String>) -> Result<(), anyhow::Error> {
    let mut client = pool.get().await.map_err(|e| {
        error!("Could not get client from pool");
        e
    })?;
    let transaction = client.transaction().await.map_err(|e| {
        error!("Could not start transaction");
        e
    })?;

    delete_product_genre().bind(&transaction, ids).await?;
    delete_product_usergenre().bind(&transaction, ids).await?;
    delete_product().bind(&transaction, ids).await?;

    transaction.commit().await.map_err(|e| {
        error!("Could not commit transaction: {}", e);
        e
    })?;

    Ok(())
}

impl From<dlsite::product::AgeRating> for crate::cornucopia::types::public::Age {
    fn from(value: dlsite::product::AgeRating) -> Self {
        use crate::cornucopia::types::public::Age;
        use dlsite::product::AgeRating;
        match value {
            AgeRating::AllAges => Age::all_ages,
            AgeRating::R => Age::r,
            AgeRating::Adult => Age::adult,
        }
    }
}
