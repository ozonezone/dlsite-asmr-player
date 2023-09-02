use dlsite::{
    search::{
        options::{Order, ProductSearchOptions},
        SearchResult,
    },
    DlsiteClient,
};
use rspc::Type;
use serde::Deserialize;

use crate::{
    interface::{BrowseQuery, ProductSortOrder},
    prisma::genre,
    Db,
};

#[derive(Deserialize, Type)]
pub enum RemoteProductSortType {
    ReleasedAt,
    Trend,
    Download,
}

fn to_dlsite_sort(order: ProductSortOrder, sort: RemoteProductSortType) -> Order {
    match (sort, order) {
        (RemoteProductSortType::ReleasedAt, ProductSortOrder::Asc) => Order::Release,
        (RemoteProductSortType::ReleasedAt, ProductSortOrder::Desc) => Order::ReleaseD,
        (RemoteProductSortType::Trend, _) => Order::Trend,
        (RemoteProductSortType::Download, ProductSortOrder::Asc) => Order::Dl,
        (RemoteProductSortType::Download, ProductSortOrder::Desc) => Order::DlD,
    }
}

pub async fn search_product(
    db: Db,
    query: BrowseQuery,
    page: i32,
    limit: i32,
    order: ProductSortOrder,
    sort: RemoteProductSortType,
) -> Result<SearchResult, anyhow::Error> {
    let client = DlsiteClient::default();

    let genres = db
        .genre()
        .find_many(vec![genre::name::in_vec(query.genres)])
        .exec()
        .await?
        .into_iter()
        .flat_map(|g| g.id.parse().ok())
        .collect::<Vec<u32>>();

    let keyword = query
        .words
        .into_iter()
        .chain(query.creators.into_iter())
        .collect::<Vec<_>>();
    let keyword = if keyword.is_empty() {
        None
    } else {
        Some(keyword.join("+"))
    };

    let res = client
        .search_product(&ProductSearchOptions {
            keyword,
            genre: if genres.is_empty() {
                None
            } else {
                Some(genres)
            },
            age_category: query.age_category.map(|c| vec![c]),
            page: Some(page as u32),
            per_page: Some(limit as u32),
            order: Some(to_dlsite_sort(order, sort)),
            ..Default::default()
        })
        .await?;
    Ok(res)
}
