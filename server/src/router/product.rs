use std::{collections::HashMap, path::PathBuf, str::FromStr};

use rspc::Type;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::cornucopia::queries::{
    genre::{get_genres, get_usergenres},
    product::{
        count_product, get_product_name_asc, get_product_name_desc, get_product_path,
        get_product_released_at_asc, get_product_released_at_desc,
    },
};

use super::{
    common::{Age, Genre, ProductDbResult, SortOrder, SortType, UserGenre},
    utils::ToRspcError,
    RouterBuilder,
};

#[derive(Type, Serialize)]
pub struct ProductResult {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub series: Option<String>,
    pub circle_id: String,
    pub actor: Vec<String>,
    pub author: Vec<String>,
    pub illustrator: Vec<String>,
    pub price: i32,
    pub sale_count: i32,
    pub age: Age,
    pub released_at: String,
    pub rating: Option<f64>,
    pub rating_count: i32,
    pub comment_count: i32,
    pub path: String,
    pub remote_image: Vec<String>,
    pub circle_name: String,
    pub genre: Vec<Genre>,
    pub user_genre: Vec<UserGenre>,
}

#[derive(Deserialize, Type)]
pub struct BrowseParams {
    sort_type: SortType,
    sort_order: SortOrder,
    page: i32,
    limit: i32,
}

pub(crate) fn mount() -> RouterBuilder {
    <RouterBuilder>::new()
        .query("browse", |t| {
            t(|ctx, params: BrowseParams| async move {
                let client = ctx
                    .pool
                    .get()
                    .await
                    .to_rspc_internal_error("Failed to get client")?;

                let offset = (params.page - 1) * params.limit;
                let result: Result<Vec<ProductDbResult>, tokio_postgres::Error> =
                    match params.sort_type {
                        SortType::Name => match params.sort_order {
                            SortOrder::Asc => get_product_name_asc()
                                .bind(&client, &params.limit.into(), &offset.into())
                                .all()
                                .await
                                .map(|res| res.into_iter().map(|res| res.into()).collect()),
                            SortOrder::Desc => get_product_name_desc()
                                .bind(&client, &params.limit.into(), &offset.into())
                                .all()
                                .await
                                .map(|res| res.into_iter().map(|res| res.into()).collect()),
                        },
                        SortType::Date => match params.sort_order {
                            SortOrder::Asc => get_product_released_at_asc()
                                .bind(&client, &params.limit.into(), &offset.into())
                                .all()
                                .await
                                .map(|res| res.into_iter().map(|res| res.into()).collect()),
                            SortOrder::Desc => get_product_released_at_desc()
                                .bind(&client, &params.limit.into(), &offset.into())
                                .all()
                                .await
                                .map(|res| res.into_iter().map(|res| res.into()).collect()),
                        },
                    };
                let result = result.to_rspc_internal_error("Failed to get products")?;

                let mut genres: HashMap<String, Vec<Genre>> = HashMap::new();
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
                let client = ctx
                    .pool
                    .get()
                    .await
                    .to_rspc_internal_error("Failed to get client")?;
                let product_folder = get_product_path()
                    .bind(&client, &product_id)
                    .one()
                    .await
                    .to_rspc_internal_error("Invalid product")?;
                let product_folder =
                    PathBuf::from_str(&product_folder).to_rspc_internal_error("Invalid path")?;
                let get_files_tasks = tokio::task::spawn_blocking(move || {
                    let mut files: Vec<Vec<String>> = vec![];
                    for entry in walkdir::WalkDir::new(&product_folder) {
                        if let Ok(entry) = &entry {
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
