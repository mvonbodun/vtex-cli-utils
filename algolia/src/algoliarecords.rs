use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ItemRecord {
    pub sku_id: i32,
    #[serde(rename="sku")]
    pub sku_ref: String,
    pub product_id: i32,
    #[serde(rename="parentID")]
    pub parent_ref: String,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub brand: String,
    pub hierarchical_categories: HierarchicalCategories,
    pub list_categories: Vec<String>,
    pub category_page_id: Vec<String>,
    pub image_urls: Vec<String>,
    pub image_blurred: Option<String>,
    pub reviews: Option<Review>,
    pub color: Option<String>,
    pub available_colors: Option<Vec<String>>,
    pub size: Option<String>,
    pub available_sizes: Option<Vec<String>>,
    pub variants: Vec<Variant>,
    pub price: Price,
    pub units_in_stock: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub related_products: Option<Vec<ItemRecord>>,
    pub product_type: Option<String>,
    #[serde(rename="objectID")]
    pub object_id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HierarchicalCategories {
    pub lvl0: String,
    pub lvl1: String,
    pub lvl2: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Review {
    pub rating: i32,
    pub count: i32,
    pub bayesian_avg: f32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Color {
    pub filter_group: String,
    pub original_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Variant {
    #[serde(rename="sku")]
    pub sku_ref: String,
    pub abbreviated_size: Option<String>,
    pub abbreviated_color: Option<String>,
    pub in_stock: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Price {
    pub currency: String,
    pub value: f32,
    pub discounted_value: f32,
    pub discount_level: f32,
    pub on_sales: bool,
}

