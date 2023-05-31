use entity::entities::product;
use rspc::Type;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Type)]
pub enum SortType {
    Name,
    Date,
}
impl From<SortType> for product::Column {
    fn from(sort_type: SortType) -> Self {
        match sort_type {
            SortType::Name => product::Column::Name,
            SortType::Date => product::Column::ReleasedAt,
        }
    }
}

#[derive(Deserialize, Type)]
pub enum SortOrder {
    Asc,
    Desc,
}
impl From<SortOrder> for sea_orm::Order {
    fn from(sort_order: SortOrder) -> Self {
        match sort_order {
            SortOrder::Asc => sea_orm::Order::Asc,
            SortOrder::Desc => sea_orm::Order::Desc,
        }
    }
}

#[derive(Serialize, Type)]
pub struct Genre {
    pub id: String,
    pub name: String,
}
#[derive(Serialize, Type)]
pub struct UserGenre {
    pub id: String,
    pub name: String,
    pub count: i32,
}
