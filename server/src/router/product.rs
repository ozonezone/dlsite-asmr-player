use std::{collections::HashMap, path::PathBuf, str::FromStr};

use entity::entities::{circle, genre, product, product_genre};
use migration::{Expr, PgFunc};
use rspc::Type;
use sea_orm::{EntityTrait, LoaderTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use tracing::warn;

use super::{
    common::{Age, Genre, ProductDbResult, SortOrder, SortType, UserGenre},
    utils::ToRspcError,
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
                let product = get_product()
                    .bind(&client, &product_id)
                    .one()
                    .await
                    .to_rspc_internal_error("Failed to get product")?;

                let genre = get_genre()
                    .bind(&client, &product.id)
                    .all()
                    .await
                    .to_rspc_internal_error("Failed to get genre")?
                    .into_iter()
                    .map(move |g| Genre {
                        name: g.name,
                        id: g.genre_id,
                    })
                    .collect::<Vec<_>>();
                let user_genre = get_usergenre()
                    .bind(&client, &product.id)
                    .all()
                    .await
                    .to_rspc_internal_error("Failed to get genre")?
                    .into_iter()
                    .map(move |g| UserGenre {
                        name: g.name,
                        id: g.genre_id,
                        count: g.count,
                    })
                    .collect::<Vec<_>>();

                Ok(ProductResult {
                    genre,
                    user_genre,
                    id: product.id,
                    name: product.name,
                    description: product.description,
                    actor: product.actor,
                    series: product.series,
                    circle_id: product.circle_id,
                    author: product.author,
                    illustrator: product.illustrator,
                    price: product.price,
                    age: match product.age {
                        crate::cornucopia::types::public::Age::adult => Age::Adult,
                        crate::cornucopia::types::public::Age::all_ages => Age::AllAges,
                        crate::cornucopia::types::public::Age::r => Age::R,
                    },
                    sale_count: product.sale_count,
                    released_at: product.released_at.to_string(),
                    rating: product.rating,
                    rating_count: product.rating_count,
                    comment_count: product.comment_count,
                    path: product.path,
                    remote_image: product.remote_image,
                    circle_name: product.circle_name,
                })
            })
        })
        .query("browse", |t| {
            t(|ctx, params: BrowseParams| async move {
                let offset = (params.page - 1) * params.limit;

                let products = product::Entity::find()
                    .order_by(params.sort_type.into(), params.sort_order.into())
                    .limit(params.limit.into())
                    .offset(offset.into())
                    .all(&ctx.pool)
                    .await?;
                let ids = products.iter().map(|p| p.id.clone()).collect::<Vec<_>>();

                let genres = products.load_many(product_genre::Entity, &ctx.pool).await?;

                get_genres()
                    .bind(
                        &client,
                        &result.iter().map(|p| p.id.clone()).collect::<Vec<_>>(),
                    )
                    .all()
                    .await
                    .to_rspc_internal_error("Failed to get genres")?
                    .into_iter()
                    .for_each(|genre| {
                        let genre_data = Genre {
                            id: genre.genre_id,
                            name: genre.name,
                        };
                        if let Some(data) = genres.get_mut(&genre.product_id) {
                            data.push(genre_data);
                        } else {
                            genres.insert(genre.product_id, vec![genre_data]);
                        }
                    });

                let mut user_genres: HashMap<String, Vec<UserGenre>> = HashMap::new();
                get_usergenres()
                    .bind(
                        &client,
                        &result.iter().map(|p| p.id.clone()).collect::<Vec<_>>(),
                    )
                    .all()
                    .await
                    .to_rspc_internal_error("Failed to get user genres")?
                    .into_iter()
                    .for_each(|genre| {
                        let genre_data = UserGenre {
                            id: genre.genre_id,
                            name: genre.name,
                            count: genre.count,
                        };
                        if let Some(data) = user_genres.get_mut(&genre.product_id) {
                            data.push(genre_data);
                        } else {
                            user_genres.insert(genre.product_id, vec![genre_data]);
                        }
                    });

                let products = result
                    .into_iter()
                    .map(|product| ProductResult {
                        genre: genres.remove(&product.id).unwrap_or_default(),
                        user_genre: user_genres.remove(&product.id).unwrap_or_default(),
                        id: product.id,
                        name: product.name,
                        description: product.description,
                        actor: product.actor,
                        series: product.series,
                        circle_id: product.circle_id,
                        author: product.author,
                        illustrator: product.illustrator,
                        price: product.price,
                        age: product.age.into(),
                        sale_count: product.sale_count,
                        released_at: product.released_at.to_string(),
                        rating: product.rating,
                        rating_count: product.rating_count,
                        comment_count: product.comment_count,
                        path: product.path,
                        remote_image: product.remote_image,
                        circle_name: product.circle_name,
                    })
                    .collect::<Vec<_>>();

                let item_count: i32 = count_product()
                    .bind(&client)
                    .one()
                    .await
                    .to_rspc_internal_error("Failed to count product")?
                    .try_into()
                    .to_rspc_internal_error("Maxium count exceeded")?;

                Ok((products, item_count))
            })
        })
        .query("files", |t| {
            t(|ctx, product_id: String| async move {
                let product_folder = product::Entity::find_by_id(product_id)
                    .select_only()
                    .column(product::Column::Path)
                    .into_tuple()
                    .one(&ctx.pool)
                    .await
                    .to_rspc_internal_error("Invalid product")?;
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
