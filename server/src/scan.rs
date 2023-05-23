use std::{path::PathBuf, sync::Arc};

use chrono::TimeZone;
use futures::stream::StreamExt;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{debug, error, info, warn};
use walkdir::WalkDir;

use crate::prisma::{self, PrismaClient};

static DLSITE_FOLDER_REGEX: Lazy<Regex> = Lazy::new(|| regex::Regex::new(r"(?i)RJ\d+").unwrap());

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

/// Scan folders for RJ folders and fetch metadata for each RJ folder.
///
/// # Arguments
/// * `folders` - List of folders to scan
/// * `force` - Force fetch metadata for each RJ folder even if the metadata already exists in db.
pub async fn scan(
    folders: &Vec<PathBuf>,
    force: bool,
    db: Arc<PrismaClient>,
) -> anyhow::Result<()> {
    info!("Starting scan");

    let found_id_and_path = scan_rj_folder(folders).await;

    let dlsite_client = dlsite::DlsiteClient::default();

    let id_paths_to_fetch = if force {
        found_id_and_path
    } else {
        let exclude_ids = db
            ._batch(found_id_and_path.iter().map(|(id, _)| {
                db.product()
                    .find_first(vec![crate::prisma::product::id::equals(id.clone())])
            }))
            .await?
            .into_iter()
            .filter_map(|res| res.map(|res| res.id))
            .collect::<Vec<_>>();
        found_id_and_path
            .into_iter()
            .filter(|(id, _)| !exclude_ids.contains(id))
            .collect::<Vec<_>>()
    };

    info!(
        "Fetching metadata for {} RJ folders",
        id_paths_to_fetch.len()
    );

    let fetch_tasks = id_paths_to_fetch.into_iter().map(|(dlsite_id, path)| {
        let dlsite_client = dlsite_client.clone();
        let db = db.clone();

        async move {
            tokio::spawn(async move {
                debug!("Fetching metadata for {}", dlsite_id);
                let data = dlsite_client.get_product(&dlsite_id).await.map_err(|err| {
                    error!("Failed to fetch metadata for {}: {}", dlsite_id, err);
                    err
                })?;

                Ok::<(dlsite::product::Product, PathBuf), anyhow::Error>((data, path))
            })
            .await
        }
    });

    let result = futures::stream::iter(fetch_tasks)
        .buffer_unordered(20)
        .collect::<Vec<_>>()
        .await;
    let fetched = result
        .into_iter()
        .filter_map(|res| {
            if let Ok(Ok(res)) = res {
                Some(res)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    db._batch(fetched.into_iter().map(|(data, path)| {
        (
            // upsert circle
            db.circle().upsert(
                prisma::circle::id::equals(data.circle.id.clone()),
                (data.circle.id.clone(), data.circle.name, vec![]),
                vec![],
            ),
            // upsert genres
            [
                data.genre.clone(),
                data.reviewer_genre
                    .iter()
                    .map(|genre| genre.0.clone())
                    .collect::<Vec<_>>(),
            ]
            .concat()
            .into_iter()
            .map(|genre| {
                db.genre().upsert(
                    prisma::genre::id::equals(genre.id.clone()),
                    (genre.id.clone(), genre.name, vec![]),
                    vec![],
                )
            })
            .collect::<Vec<_>>(),
            // upsert ProductUserGenre
            data.reviewer_genre
                .iter()
                .map(|genre| {
                    db.product_user_genre().create(
                        genre.1.try_into().unwrap(),
                        prisma::product::id::equals(data.id.clone()),
                        prisma::genre::id::equals(genre.0.id.clone()),
                        vec![],
                    )
                })
                .collect::<Vec<_>>(),
            // create products
            db.product().create(
                data.id.clone(),
                data.title,
                prisma::circle::id::equals(data.circle.id),
                data.price.try_into().unwrap(),
                data.sale_count.try_into().unwrap(),
                data.age_rating.into(),
                chrono::FixedOffset::east_opt(9 * 3600)
                    .unwrap()
                    .from_local_datetime(&data.released_at.and_hms_opt(0, 0, 0).unwrap())
                    .unwrap(),
                data.rate_count.unwrap_or(0).try_into().unwrap(),
                data.review_count.unwrap_or(0).try_into().unwrap(),
                path.to_string_lossy().to_string(),
                vec![
                    prisma::product::actor::set(data.people.voice_actor.unwrap_or_default()),
                    prisma::product::illustrator::set(data.people.illustrator.unwrap_or_default()),
                    prisma::product::author::set(data.people.author.unwrap_or_default()),
                    prisma::product::rating::set(data.rating),
                ],
            ),
        )
    }))
    .await
    .map_err(|err| {
        error!("Could not write to db: {}", err);
        err
    })?;

    info!("Scan finished.");

    Ok(())
}

impl From<dlsite::product::AgeRating> for prisma::ProductAge {
    fn from(age: dlsite::product::AgeRating) -> Self {
        match age {
            dlsite::product::AgeRating::AllAges => prisma::ProductAge::AllAge,
            dlsite::product::AgeRating::Adult => prisma::ProductAge::Adult,
            dlsite::product::AgeRating::R => prisma::ProductAge::RRated,
        }
    }
}
