use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProductLookup {
    pub part_number: String,
    pub product_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkuLookup {
    pub part_number: String,
    pub sku_id: i32,
}
