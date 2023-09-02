use dlsite::interface::AgeCategory;
use prisma_client_rust::{or, queries::Result as DbResult, Direction};

use crate::{
    interface::{ProductSortOrder, ProductSortType},
    prisma::{circle, creator, genre, product, product_creator, product_genre},
    Db,
};

use super::product_detailed;

fn to_db_sort(
    sort_type: ProductSortType,
    sort_order: ProductSortOrder,
) -> <product::Types as prisma_client_rust::ModelTypes>::OrderBy {
    let direction = match sort_order {
        ProductSortOrder::Desc => Direction::Desc,
        ProductSortOrder::Asc => Direction::Asc,
    };
    match sort_type {
        ProductSortType::Name => product::title::order(direction),
        ProductSortType::ReleasedAt => product::released_at::order(direction),
        ProductSortType::CreatedAt => product::created_at::order(direction),
    }
}

#[derive(Debug)]
pub struct BrowseQuery {
    pub words: Vec<String>,
    pub genres: Vec<String>,
    pub circles: Vec<String>,
    pub creators: Vec<String>,
    pub age_category: Option<AgeCategory>,
}

pub async fn browse(
    db: Db,
    query: BrowseQuery,
    page: i32,
    limit: i32,
    order: ProductSortOrder,
    sort: ProductSortType,
) -> DbResult<(Vec<product_detailed::Data>, i64)> {
    // TODO: Make and search works
    let search_query = query
        .words
        .into_iter()
        .map(product::title::contains)
        .chain(query.genres.into_iter().map(|genre| {
            product::genres::some(vec![product_genre::genre::is(vec![genre::name::equals(
                genre,
            )])])
        }))
        .chain(
            query
                .circles
                .into_iter()
                .map(|circle| product::circle::is(vec![circle::name::equals(circle)])),
        )
        .chain(query.creators.into_iter().map(|creator| {
            product::creators::some(vec![product_creator::creator::is(vec![
                creator::name::equals(creator),
            ])])
        }))
        .chain(
            std::iter::once(
                query
                    .age_category
                    .map(|age_category| product::age::equals(age_category.into())),
            )
            .flatten(),
        )
        .collect::<Vec<_>>();

    let count = db.product().count(search_query.clone()).exec().await?;

    let products = db
        .product()
        .find_many(search_query)
        .skip(((page - 1) * limit).into())
        .order_by(to_db_sort(sort, order))
        .take(limit.into())
        .include(product_detailed::include())
        .exec()
        .await?;

    Ok((products, count))
}
