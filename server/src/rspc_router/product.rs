use std::{collections::HashMap, path::PathBuf, str::FromStr};

use entity::entities::{circle, genre, product, product_genre, product_user_genre};
use rspc::Type;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::Deserialize;
use tracing::warn;

use super::{
    common::{Genre, ProductResponse, SortOrder, SortType, UserGenre},
    utils::{ToRspcInternalError, ToRspcNotFound},
    RouterBuilder,
};

#[derive(Deserialize, Type)]
pub struct BrowseParams {
    sort_type: SortType,
    sort_order: SortOrder,
    page: u32,
    limit: u32,
}

pub(crate) fn mount() -> RouterBuilder {
    <RouterBuilder>::new()
        .query("get", |t| {
            t(|ctx, product_id: String| async move {
                let product = product::Entity::find()
                    .filter(product::Column::Id.eq(product_id.clone()))
                    .one(&ctx.db)
                    .await
                    .to_rspc_internal_error("Failed to find product")?
                    .to_rspc_not_found("No product found")?;

                let circle_name = circle::Entity::find_by_id(product.circle_id.clone())
                    .one(&ctx.db)
                    .await
                    .to_rspc_internal_error("Failed to find circle")?
                    .to_rspc_not_found("No circle found")?
                    .name;

                let genre = product_genre::Entity::find()
                    .find_also_related(genre::Entity)
                    .filter(product_genre::Column::ProductId.eq(product_id.clone()))
                    .all(&ctx.db)
                    .await
                    .to_rspc_internal_error("Failed to get genres")?
                    .into_iter()
                    .filter_map(|(_, g)| {
                        if let Some(g) = g {
                            Some(Genre {
                                id: g.id,
                                name: g.name,
                            })
                        } else {
                            warn!("Genre not found for product {}", product_id);
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                let user_genre = product_user_genre::Entity::find()
                    .find_also_related(genre::Entity)
                    .filter(product_user_genre::Column::ProductId.eq(product_id.clone()))
                    .all(&ctx.db)
                    .await
                    .to_rspc_internal_error("Failed to get user genres")?
                    .into_iter()
                    .filter_map(|(pg, g)| {
                        if let Some(g) = g {
                            Some(UserGenre {
                                id: g.id,
                                name: g.name,
                                count: pg.count,
                            })
                        } else {
                            warn!("Genre not found for product {}", product_id);
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                Ok(ProductResponse {
                    product,
                    genre,
                    user_genre,
                    circle_name,
                })
            })
        })
        .query("browse", |t| {
            t(|ctx, params: BrowseParams| async move {
                let offset = (params.page - 1) * params.limit;

                let products = product::Entity::find()
                    .order_by(
                        product::Column::from(params.sort_type),
                        params.sort_order.into(),
                    )
                    .limit(u64::from(params.limit))
                    .offset(u64::from(offset))
                    .find_also_related(circle::Entity)
                    .all(&ctx.db)
                    .await
                    .to_rspc_internal_error("Failed to get products")?;
                let ids = products.iter().map(|p| p.0.id.clone()).collect::<Vec<_>>();

                let mut genres_map = HashMap::new();
                product_genre::Entity::find()
                    .find_also_related(genre::Entity)
                    .filter(product_genre::Column::ProductId.is_in(ids.clone()))
                    .all(&ctx.db)
                    .await
                    .to_rspc_internal_error("Failed to get genres")?
                    .into_iter()
                    .for_each(|(pg, g)| {
                        if let Some(g) = g {
                            genres_map
                                .entry(pg.product_id)
                                .or_insert_with(Vec::new)
                                .push(Genre {
                                    id: g.id,
                                    name: g.name,
                                });
                        }
                    });
                let mut user_genres_map = HashMap::new();
                product_user_genre::Entity::find()
                    .find_also_related(genre::Entity)
                    .filter(product_user_genre::Column::ProductId.is_in(ids.clone()))
                    .all(&ctx.db)
                    .await
                    .to_rspc_internal_error("Failed to get user genres")?
                    .into_iter()
                    .for_each(|(pg, g)| {
                        if let Some(g) = g {
                            user_genres_map
                                .entry(pg.product_id.clone())
                                .or_insert_with(Vec::new)
                                .push(UserGenre {
                                    id: g.id,
                                    name: g.name,
                                    count: pg.count,
                                });
                        }
                    });

                let products = products
                    .into_iter()
                    .map(|product| ProductResponse {
                        genre: genres_map.remove(&product.0.id).unwrap_or_default(),
                        user_genre: user_genres_map.remove(&product.0.id).unwrap_or_default(),
                        product: product.0,
                        circle_name: product.1.map(|p| p.name).unwrap_or_else(|| "".to_string()),
                    })
                    .collect::<Vec<_>>();

                let item_count: i32 = product::Entity::find()
                    .count(&ctx.db)
                    .await
                    .to_rspc_internal_error("Failed to count items")?
                    .try_into()
                    .to_rspc_internal_error("Out of range")?;
                Ok((products, item_count))
            })
        })
        .query("files", |t| {
            t(|ctx, product_id: String| async move {
                let product_folder: String = product::Entity::find_by_id(product_id)
                    .select_only()
                    .column(product::Column::Path)
                    .into_tuple()
                    .one(&ctx.db)
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
