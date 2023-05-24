use std::path::PathBuf;

use chrono::TimeZone;
use futures::stream::StreamExt;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{error, info, warn};
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
pub async fn scan(folders: &Vec<PathBuf>, force: bool, db: &PrismaClient) -> anyhow::Result<()> {
    info!("Starting scan");

    let id_paths = scan_rj_folder(folders).await;

    let id_to_fetch = if force {
        let ids: Vec<String> = db
            .product()
            .find_many(
                id_paths
                    .iter()
                    .map(|(id, _)| crate::prisma::product::id::equals(id.to_string()))
                    .collect(),
            )
            .select(prisma::product::select!({ id }))
            .exec()
            .await?
            .into_iter()
            .map(|res| res.id)
            .collect();

        id_paths
            .into_iter()
            .filter(|(id, _)| !ids.contains(id))
            .collect::<Vec<_>>()
    } else {
        id_paths
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

    let batch = metadata
        .into_iter()
        .map(|(data, path)| {
            dbg!(&data.circle.id);
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
                            vec![prisma::product_user_genre::vote_count::set(
                                genre.1.try_into().unwrap(),
                            )],
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
                        prisma::product::illustrator::set(
                            data.people.illustrator.unwrap_or_default(),
                        ),
                        prisma::product::author::set(data.people.author.unwrap_or_default()),
                        prisma::product::rating::set(data.rating),
                    ],
                ),
            )
        })
        .collect::<Vec<_>>();

    let res = db._batch(batch).await.map_err(|e| {
        error!("Failed to save metadata to database: {}", e);
        e
    })?;

    info!("Scan finished");

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
