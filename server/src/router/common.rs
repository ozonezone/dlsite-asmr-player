use rspc::Type;
use serde::{Deserialize, Serialize};
use struct_convert::Convert;

use crate::cornucopia::queries::{
    circle::{
        GetCircleProductNameAsc, GetCircleProductNameDesc, GetCircleProductReleasedAtAsc,
        GetCircleProductReleasedAtDesc,
    },
    product::{
        GetProductNameAsc, GetProductNameDesc, GetProductReleasedAtAsc, GetProductReleasedAtDesc,
    },
};

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
#[convert(from = "GetCircleProductNameAsc")]
#[convert(from = "GetCircleProductNameDesc")]
#[convert(from = "GetCircleProductReleasedAtDesc")]
#[convert(from = "GetCircleProductReleasedAtAsc")]
pub struct ProductDbResult {
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
    pub age: crate::cornucopia::types::public::Age,
    pub released_at: time::Date,
    pub rating: Option<f64>,
    pub rating_count: i32,
    pub comment_count: i32,
    pub path: String,
    pub remote_image: Vec<String>,
    pub circle_name: String,
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
#[derive(Serialize, Type)]
pub enum Age {
    AllAges,
    R,
    Adult,
}

impl From<crate::cornucopia::types::public::Age> for Age {
    fn from(age: crate::cornucopia::types::public::Age) -> Self {
        match age {
            crate::cornucopia::types::public::Age::adult => Age::Adult,
            crate::cornucopia::types::public::Age::all_ages => Age::AllAges,
            crate::cornucopia::types::public::Age::r => Age::R,
        }
    }
}
