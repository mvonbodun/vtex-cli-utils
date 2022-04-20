use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CatRecord {
    pub group_identifier: String,
    pub parent_group_identifier: String,
    pub top_group: bool,
    pub sequence: i32,
    pub name: String,
    pub short_description: String,
    pub long_description: String,
    pub thumbnail: String,
    pub full_image: String,
    pub field1: String,
    pub published: i32,
    pub delete: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProdHeaderRecord {
    pub part_number: String,
    pub parent_part_number: String,
    #[serde(rename = "Type")]
    pub product_type: String,
    pub list_price: f32,
    pub parent_group_identifier: String,
    pub sequence: i32,
    #[serde(rename = "Language_ID")]
    pub language_id: i32,
    pub name: String,
    pub short_description: String,
    pub long_description: String,
    #[serde(rename = "IPSID")]
    pub ipsid: String,
    pub buyable: i32,
    pub available: i32,
    pub published: i32,
    pub keyword: String,
    pub start_date: String,
    pub delete: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProdDescAttrRecord {
    pub part_number: String,
    pub usage: i32,
    pub name: String,
    pub value: String,
    pub sequence: i32,
    pub delete: i32,
    pub group_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProdAttrAllowedValuesRecord {
    pub part_number: String,
    #[serde(rename = "Type")]
    pub product_type: String,
    pub name: String,
    pub sequence: i32,
    pub allowed_value1: String,
    pub allowed_value2: String,
    pub allowed_value3: String,
    pub allowed_value4: String,
    pub allowed_value5: String,
    pub allowed_value6: String,
    pub allowed_value7: String,
    pub allowed_value8: String,
    pub allowed_value9: String,
    pub allowed_value10: String,
    pub allowed_value11: String,
    pub allowed_value12: String,
    pub allowed_value13: String,
    pub allowed_value14: String,
    pub allowed_value15: String,
    pub allowed_value16: String,
    pub allowed_value17: String,
    pub allowed_value18: String,
    pub allowed_value19: String,
    pub allowed_value20: String,
    pub allowed_value21: String,
    pub allowed_value22: String,
    pub allowed_value23: String,
    pub allowed_value24: String,
    pub allowed_value25: String,
    pub allowed_value26: String,
    pub allowed_value27: String,
    pub allowed_value28: String,
    pub allowed_value29: String,
    pub allowed_value30: String,
    pub allowed_value31: String,
    pub allowed_value32: String,
    pub allowed_value33: String,
    pub allowed_value34: String,
    pub allowed_value35: String,
    pub allowed_value36: String,
    pub allowed_value37: String,
    pub allowed_value38: String,
    pub allowed_value39: String,
    pub allowed_value40: String,
    pub allowed_value41: String,
    pub allowed_value42: String,
    pub allowed_value43: String,
    pub allowed_value44: String,
    pub allowed_value45: String,
    pub allowed_value46: String,
    pub allowed_value47: String,
    pub allowed_value48: String,
    pub allowed_value49: String,
    pub allowed_value50: String,
    pub allowed_value51: String,
    pub allowed_value52: String,
    pub allowed_value53: String,
    pub allowed_value54: String,
    pub allowed_value55: String,
    pub allowed_value56: String,
    pub allowed_value57: String,
    pub allowed_value58: String,
    pub allowed_value59: String,
    pub allowed_value60: String,
    pub delete: i32,
}

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkuDefineAttrValueRecord {
    pub part_number: String,
    pub name: String,
    pub value: String,
    pub delete: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PricingRecord {
    pub catentry_part_number: String,
    pub price: f32,
    pub currency_code: String,
    pub start_date: String,
    pub end_date: String,
    pub precedence: i32,
    pub minimum_quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InventoryRecord {
    pub part_number: String,
    pub quantity: i32,
    pub backorderable: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkusInactive {
    pub sku_id: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ProductSpecificationAssignmentAlternate {
    pub product_ref_id: i32,
    pub short_desc: Option<String>,
    #[serde(rename = "ship_message")]
    pub ship_message: Option<String>,
    #[serde(rename = "Availability Remarks")]
    pub availability_remarks: Option<String>,
    pub weight: Option<String>,
    #[serde(rename = "Package Dimensions")]
    pub package_dimensions: Option<String>,
    #[serde(rename = "Shipping Remarks")]
    pub shipping_remarks: Option<String>,
    #[serde(rename = "Prop65")]
    pub prop_65: Option<String>,
    pub attachment: Option<String>,
}
