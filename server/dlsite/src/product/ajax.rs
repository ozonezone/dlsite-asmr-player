use std::collections::HashMap;

use serde::Deserialize;

use crate::{DlsiteError, Result};

#[derive(Default, Debug, Clone, Deserialize)]
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
    pub work_type: String,
}

pub(super) fn parse_product_json(json_str: &str, product_id: &str) -> Result<ProductAjax> {
    let mut json: HashMap<String, ProductAjax> = serde_json::from_str(json_str)?;
    let product = std::mem::replace(&mut json.get_mut(product_id), None)
        .ok_or_else(|| DlsiteError::ParseError("Failed to parse json".to_string()))?;
    let product = std::mem::take(product);
    Ok(product)
}
