use crate::{product::Product, DlsiteClient, Result};

pub struct SearchProductEntry {
    pub id: String,
    pub title: String,
}

impl DlsiteClient {
    pub async fn search_product(&self, keyword: &str) -> Result<Vec<SearchProductEntry>> {
        unimplemented!()
    }

    pub async fn get_full_product(&self, entry: SearchProductEntry) -> Result<Product> {
        self.get_product(&entry.id).await
    }
}
