use std::path::PathBuf;

use chrono::{Datelike, TimeZone};
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use futures::stream::StreamExt;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{error, info, warn};
use walkdir::WalkDir;

use crate::cornucopia::{
    queries::insert_product::{exist_product, insert_product, InsertProductParams},
    types::public::Age,
};

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
pub async fn scan(folders: &Vec<PathBuf>, force: bool, pool: &Pool) -> anyhow::Result<()> {
    info!("Starting scan");

    let client = pool.get().await?;

    let id_paths = scan_rj_folder(folders).await;

    let id_to_fetch = if force {
        id_paths
    } else {
        let find_ids = id_paths
            .iter()
            .map(|(id, _)| id.to_string())
            .collect::<Vec<_>>();
        let db_available_id = exist_product().bind(&client, &find_ids).all().await?;
        id_paths
            .into_iter()
            .filter(|(id, _)| {
                !db_available_id
                    .iter()
                    .any(|db_id| db_id.to_uppercase() == id.to_uppercase())
            })
            .collect::<Vec<_>>()
    };

    let dlsite_client = dlsite::DlsiteClient::default();
    let fetch_tasks = id_to_fetch.into_iter().map(|data| {
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

    info!("Fetching metadata for {} RJ folders", fetch_tasks.len());

    let mut metadata = futures::stream::iter(fetch_tasks)
        .buffer_unordered(20)
        .collect::<Vec<_>>()
        .await;
    // Remove data with error
    let metadata = metadata
        .drain(..)
        .filter_map(|res| res.ok())
        .filter_map(|res| res.0.ok().map(|data| (data, res.1)))
        .collect::<Vec<_>>();

    info!("Fetched metadata for {} RJ folders", metadata.len());

    let tasks = metadata.into_iter().map(|(metadata, path)| async move {
        let mut client = pool.get().await?;
        client.transaction().await?;
        insert_product().params(
            &client,
            &InsertProductParams {
                id: metadata.id,
                name: metadata.title,
                description: None::<&str>,
                series: None::<&str>,
                circle_id: metadata.circle.id,
                actor: metadata.people.voice_actor.unwrap_or_default(),
                author: metadata.people.author.unwrap_or_default(),
                illustrator: metadata.people.illustrator.unwrap_or_default(),
                price: metadata.price.try_into().unwrap(),
                sale_count: metadata.sale_count.try_into().unwrap(),
                age: metadata.age_rating.into(),
                // convert chrono date to "time" crate date
                released_at: time::Date::from_calendar_date(
                    metadata.released_at.year(),
                    time::Month::try_from(u8::try_from(metadata.released_at.month()).unwrap())
                        .unwrap(),
                    metadata.released_at.day().try_into().unwrap(),
                )
                .unwrap(),
                rating: metadata.rating,
                rating_count: metadata.rate_count.unwrap_or(0).try_into().unwrap(),
                comment_count: metadata.review_count.unwrap_or(0).try_into().unwrap(),
                path: path.to_string_lossy(),
            },
        );

        client.commit().await?;

        Ok::<(), anyhow::Error>(())
    });

    let res = futures::future::join_all(tasks).await;

    info!("Scan finished");

    Ok(())
}

impl From<dlsite::product::AgeRating> for Age {
    fn from(value: dlsite::product::AgeRating) -> Self {
        use dlsite::product::AgeRating;
        match value {
            AgeRating::AllAges => Age::all_ages,
            AgeRating::R => Age::r,
            AgeRating::Adult => Age::adult,
        }
    }
}
