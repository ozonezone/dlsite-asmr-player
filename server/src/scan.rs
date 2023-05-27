use std::path::PathBuf;

use deadpool_postgres::Pool;
use futures::stream::StreamExt;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{debug, error, info, warn};
use walkdir::WalkDir;

use crate::{
    cornucopia::queries::product::product_ids,
    db::product::{create_product, delete_product_and_relations},
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
