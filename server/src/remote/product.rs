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
    _db: Db,
    query: BrowseQuery,
    page: i32,
    limit: i32,
    order: ProductSortOrder,
    sort: RemoteProductSortType,
) -> Result<SearchResult, dlsite::DlsiteError> {
    let client = DlsiteClient::default();
    client
        .search_product(&ProductSearchOptions {
            keyword: if query.words.is_empty() {
                None
            } else {
                Some(query.words.join("+"))
            },
            page: Some(page as u32),
            per_page: Some(limit as u32),
            order: Some(to_dlsite_sort(order, sort)),
            ..Default::default()
        })
        .await
}
