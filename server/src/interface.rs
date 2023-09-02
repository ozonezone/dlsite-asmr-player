use dlsite::interface::AgeCategory;
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

#[derive(Debug)]
pub struct BrowseQuery {
    pub words: Vec<String>,
    pub genres: Vec<String>,
    pub circles: Vec<String>,
    pub creators: Vec<String>,
    pub age_category: Option<AgeCategory>,
}

impl From<dlsite::interface::AgeCategory> for crate::prisma::AgeCategory {
    fn from(value: dlsite::interface::AgeCategory) -> Self {
        match value {
            dlsite::interface::AgeCategory::General => Self::General,
            dlsite::interface::AgeCategory::R15 => Self::R15,
            dlsite::interface::AgeCategory::Adult => Self::Adult,
        }
    }
}
