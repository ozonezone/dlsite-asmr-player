use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use crate::{DlsiteClient, DlsiteError, Result};

use super::WorkType;

#[derive(Debug, Clone, Deserialize)]
pub struct ProductAjax {
    pub maker_id: String,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]
    pub dl_count: u64,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_option_number_from_string")]
    pub review_count: Option<u64>,
    pub rate_average_2dp: Option<f64>,
    pub rate_count: Option<u64>,
    pub work_name: String,
    pub price: u64,
    #[serde(deserialize_with = "deserialize_work_type")]
    pub work_type: WorkType,
}

fn deserialize_work_type<'de, D>(deserializer: D) -> std::result::Result<WorkType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    Ok(match &*s {
        "SOU" => WorkType::Voice,
        _ => WorkType::Unknown,
    })
}

impl DlsiteClient {
    #[async_backtrace::framed]
    pub(super) async fn get_product_ajax(&self, product_id: &str) -> Result<ProductAjax> {
        let path = format!("/work/=/product_id/{}", product_id);
        let ajax_json_str = self.get(&path).await?;
        let mut json: HashMap<String, ProductAjax> = serde_json::from_str(&ajax_json_str)?;
        let product = json
            .remove(product_id)
            .ok_or_else(|| DlsiteError::ParseError("Failed to parse ajax json".to_string()))?;

        Ok(product)
    }
}
