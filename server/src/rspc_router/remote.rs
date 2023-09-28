use rspc::Type;
use serde::{Deserialize, Serialize};

use crate::{
    browse::parse_query,
    interface::ProductSortOrder,
    prisma::AgeCategory,
    remote::product::{search_product, RemoteProductSortType},
};

use super::{utils::ToRspcInternalError, RouterBuilder};

#[derive(Deserialize, Type)]
pub struct RemoteSearchParams {
    sort_type: RemoteProductSortType,
    sort_order: ProductSortOrder,
    page: u32,
    limit: u32,
    query: String,
}

#[derive(Deserialize, Type, Serialize)]
pub struct RemoteSearchResponseItem {
    pub id: String,
    pub title: String,
    pub circle_name: String,
    pub circle_id: String,
    pub dl_count: Option<i32>,
    pub rate_count: Option<i32>,
    pub review_count: Option<i32>,
    pub price_original: i32,
    pub price_sale: Option<i32>,
    pub age_category: AgeCategory,
    pub work_type: String,
    pub thumbnail_url: String,
}

#[derive(Deserialize, Type, Serialize)]
pub struct RemoteSearchResponse {
    pub products: Vec<RemoteSearchResponseItem>,
    pub count: i32,
    pub query_path: String,
}

pub(crate) fn mount() -> RouterBuilder {
    <RouterBuilder>::new().query("search", |t| {
        t(|ctx, params: RemoteSearchParams| async move {
            let query = parse_query(params.query);
            let res = search_product(
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
                dlsite::interface::WorkType::SOU,
            )
            .await
            .to_rspc_internal_error("Error")?;

            Ok(RemoteSearchResponse {
                products: res
                    .products
                    .into_iter()
                    .map(|p| RemoteSearchResponseItem {
                        id: p.id,
                        title: p.title,
                        circle_name: p.circle_name,
                        circle_id: p.circle_id,
                        dl_count: p.dl_count,
                        rate_count: p.rate_count,
                        review_count: p.review_count,
                        price_original: p.price_original,
                        price_sale: p.price_sale,
                        age_category: p.age_category.into(),
                        work_type: p.work_type.to_string(),
                        thumbnail_url: p.thumbnail_url,
                    })
                    .collect(),
                count: res.count,
                query_path: res.query_path,
            })
        })
    })
}
