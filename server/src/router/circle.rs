use std::collections::HashMap;

use rspc::Type;
use serde::{Deserialize, Serialize};

use crate::cornucopia::queries::{
    circle::{
        count_circle_product, get_circle_product_name_asc, get_circle_product_name_desc,
        get_circle_product_released_at_asc, get_circle_product_released_at_desc,
    },
    genre::{get_genres, get_usergenres},
};

use super::{
    common::{Age, Genre, ProductDbResult, SortOrder, SortType, UserGenre},
    utils::ToRspcError,
    RouterBuilder,
};

#[derive(Type, Serialize)]
pub struct CircleProductResult {
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
pub struct CircleBrowseParams {
    sort_type: SortType,
    sort_order: SortOrder,
    page: i32,
    limit: i32,
    circle_id: String,
}

pub(crate) fn mount() -> RouterBuilder {
    <RouterBuilder>::new().query("browse", |t| {
        t(|ctx, params: CircleBrowseParams| async move {
            let client = ctx
                .pool
                .get()
                .await
                .to_rspc_internal_error("Failed to get db client")?;

            let offset = (params.page - 1) * params.limit;
            let result: Result<Vec<ProductDbResult>, tokio_postgres::Error> = match params.sort_type
            {
                SortType::Name => match params.sort_order {
                    SortOrder::Asc => get_circle_product_name_asc()
                        .bind(
                            &client,
                            &params.circle_id,
                            &params.limit.into(),
                            &offset.into(),
                        )
                        .all()
                        .await
                        .map(|res| res.into_iter().map(|res| res.into()).collect()),
                    SortOrder::Desc => get_circle_product_name_desc()
                        .bind(
                            &client,
                            &params.circle_id,
                            &params.limit.into(),
                            &offset.into(),
                        )
                        .all()
                        .await
                        .map(|res| res.into_iter().map(|res| res.into()).collect()),
                },
                SortType::Date => match params.sort_order {
                    SortOrder::Asc => get_circle_product_released_at_asc()
                        .bind(
                            &client,
                            &params.circle_id,
                            &params.limit.into(),
                            &offset.into(),
                        )
                        .all()
                        .await
                        .map(|res| res.into_iter().map(|res| res.into()).collect()),
                    SortOrder::Desc => get_circle_product_released_at_desc()
                        .bind(
                            &client,
                            &params.circle_id,
                            &params.limit.into(),
                            &offset.into(),
                        )
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
                .map(|product| CircleProductResult {
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

            let item_count: i32 = count_circle_product()
                .bind(&client, &params.circle_id)
                .one()
                .await
                .to_rspc_internal_error("Failed to count product")?
                .try_into()
                .to_rspc_internal_error("Maxium count exceeded")?;

            Ok((products, item_count))
        })
    })
}
