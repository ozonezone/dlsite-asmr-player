use std::collections::HashMap;

use rspc::Type;
use serde::{Deserialize, Serialize};
use struct_convert::Convert;

use crate::cornucopia::queries::{
    genre::{get_genres, get_usergenres},
    product::{
        count_product, get_product_name_asc, get_product_name_desc, get_product_released_at_asc,
        get_product_released_at_desc, GetProductNameAsc, GetProductNameDesc,
        GetProductReleasedAtAsc, GetProductReleasedAtDesc,
    },
};

use super::{utils::ToRspcError, RouterBuilder};

#[derive(Deserialize, Type)]
pub enum SortType {
    Name,
    Date,
}

#[derive(Deserialize, Type)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Convert)]
#[convert(from = "GetProductNameAsc")]
#[convert(from = "GetProductNameDesc")]
#[convert(from = "GetProductReleasedAtDesc")]
#[convert(from = "GetProductReleasedAtAsc")]
pub struct ProductDbResult {
    pub id: String,
    pub name: String,
    pub description: String,
    pub series: String,
    pub circle_id: String,
    pub actor: Vec<String>,
    pub author: Vec<String>,
    pub illustrator: Vec<String>,
    pub price: i32,
    pub sale_count: i32,
    pub age: crate::cornucopia::types::public::Age,
    pub released_at: time::Date,
    pub rating: f64,
    pub rating_count: i32,
    pub comment_count: i32,
    pub path: String,
    pub remote_image: Vec<String>,
    pub circle_name: String,
}
#[derive(Serialize, Type)]
pub struct Genre {
    id: String,
    name: String,
}
#[derive(Serialize, Type)]
pub struct UserGenre {
    id: String,
    name: String,
    count: i32,
}
#[derive(Serialize, Type)]
pub enum Age {
    AllAges,
    R,
    Adult,
}
#[derive(Type, Serialize)]
pub struct ProductResult {
    pub id: String,
    pub name: String,
    pub description: String,
    pub series: String,
    pub circle_id: String,
    pub actor: Vec<String>,
    pub author: Vec<String>,
    pub illustrator: Vec<String>,
    pub price: i32,
    pub sale_count: i32,
    pub age: Age,
    pub released_at: String,
    pub rating: f64,
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
    <RouterBuilder>::new().query("browse", |t| {
        t(|ctx, params: BrowseParams| async move {
            let client = ctx.pool.get().await.map_err(|e| {
                rspc::Error::new(
                    rspc::ErrorCode::InternalServerError,
                    format!("Failed to get db client: {}", e),
                )
            })?;

            let offset = params.page * params.limit;
            let result: Result<Vec<ProductDbResult>, tokio_postgres::Error> = match params.sort_type
            {
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
}
