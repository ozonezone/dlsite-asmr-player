use rspc::Type;
use serde::Deserialize;

#[derive(Deserialize, Type)]
pub enum ProductSortType {
    Name,
    Date,
}

#[derive(Deserialize, Type)]
pub enum ProductSortOrder {
    Asc,
    Desc,
}
