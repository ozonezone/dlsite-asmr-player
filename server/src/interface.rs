use rspc::Type;
use serde::Deserialize;

#[derive(Deserialize, Type)]
pub enum ProductSortType {
    Name,
    ReleasedAt,
    CreatedAt,
}

#[derive(Deserialize, Type)]
pub enum ProductSortOrder {
    Asc,
    Desc,
}
