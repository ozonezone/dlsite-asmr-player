use std::{path::PathBuf, str::FromStr};

use rspc::Type;
use serde::Deserialize;
use tracing::warn;

use crate::{
    browse::parse_query,
    db::product::{
        read::{get_product, get_product_folder},
        read_browse::browse,
    },
    interface::{ProductSortOrder, ProductSortType},
};

use super::{
    utils::{ToRspcInternalError, ToRspcNotFound},
    RouterBuilder,
};

#[derive(Deserialize, Type)]
pub struct BrowseParams {
    sort_type: ProductSortType,
    sort_order: ProductSortOrder,
    page: u32,
    limit: u32,
    query: String,
}

pub(crate) fn mount() -> RouterBuilder {
    <RouterBuilder>::new()
        .query("get", |t| {
            t(|ctx, product_id: String| async move {
                let product = get_product(ctx.db, product_id)
                    .await
                    .to_rspc_internal_error("Error")?
                    .to_rspc_not_found("No product found")?;

                Ok(product)
            })
        })
        .query("browse", |t| {
            t(|ctx, params: BrowseParams| async move {
                let query = parse_query(params.query);
                let (products, count) = browse(
                    ctx.db,
                    query,
                    params
                        .page
                        .try_into()
                        .to_rspc_internal_error("Invalid page")?,
                    params
                        .limit
                        .try_into()
                        .to_rspc_internal_error("Invalid limit")?,
                    params.sort_order,
                    params.sort_type,
                )
                .await
                .to_rspc_internal_error("Error")?;

                let count: i32 = count.try_into().to_rspc_internal_error("Invalid count")?;

                Ok((products, count))
            })
        })
        .query("files", |t| {
            t(|ctx, product_id: String| async move {
                let product_folder: String = get_product_folder(ctx.db, product_id)
                    .await
                    .to_rspc_internal_error("Invalid product")?
                    .to_rspc_not_found("No product found")?;
                let product_folder =
                    PathBuf::from_str(&product_folder).to_rspc_internal_error("Invalid path")?;
                let get_files_tasks = tokio::task::spawn_blocking(move || {
                    let mut files: Vec<Vec<String>> = vec![];
                    for entry in walkdir::WalkDir::new(&product_folder) {
                        if let Ok(entry) = &entry {
                            if entry.file_type().is_dir() {
                                continue;
                            }
                            if let Ok(relative_path) = entry.path().strip_prefix(&product_folder) {
                                if let Some(relative_path) = relative_path
                                    .iter()
                                    .map(|p| p.to_str().map(|p| p.to_string()))
                                    .collect::<Option<Vec<_>>>()
                                {
                                    files.push(relative_path);
                                    continue;
                                }
                            }
                        }
                        warn!("Failed to get path: {:?}", entry);
                    }

                    Ok::<Vec<Vec<String>>, anyhow::Error>(files)
                })
                .await;
                let files = get_files_tasks
                    .to_rspc_internal_error("Failed to get files: Join error")?
                    .to_rspc_internal_error("Failed to get files: Blocking error")?;

                Ok(files)
            })
        })
}
