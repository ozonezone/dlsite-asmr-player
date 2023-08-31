use prisma_client_rust::Direction;
use rspc::Type;
use serde::Deserialize;

use crate::prisma::product;

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

pub fn to_db_sort(
    sort_type: SortType,
    sort_order: SortOrder,
) -> <product::Types as prisma_client_rust::ModelTypes>::OrderBy {
    let direction = match sort_order {
        SortOrder::Desc => Direction::Desc,
        SortOrder::Asc => Direction::Asc,
    };
    match sort_type {
        SortType::Name => product::title::order(direction),
        SortType::Date => product::released_at::order(direction),
    }
}
