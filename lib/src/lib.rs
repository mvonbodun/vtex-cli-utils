pub mod model {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    pub struct Category {
        #[serde(rename = "Id")]
        pub id: Option<i32>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(rename = "FatherCategoryId")]
        pub father_category_id: Option<i32>,
        #[serde(rename = "Title")]
        pub title: String,
        #[serde(rename = "Description")]
        pub description: String,
        #[serde(rename = "Keywords")]
        pub keywords: String,
        #[serde(rename = "IsActive")]
        pub is_active: bool,
        #[serde(rename = "LomadeeCampaignCode")]
        pub lomadee_campaign_code: Option<String>,
        #[serde(rename = "AdWordsRemarketingCode")]
        pub ad_words_remarketing_code: Option<String>,
        #[serde(rename = "ShowInStoreFront")]
        pub show_in_store_front: bool,
        #[serde(rename = "ShowBrandFilter")]
        pub show_brand_filter: bool,
        #[serde(rename = "ActiveStoreFrontLink")]
        pub active_store_front_link: bool,
        #[serde(rename = "GlobalCategoryId")]
        pub global_category_id: Option<i32>,
        #[serde(rename = "StockKeepingUnitSelectionMode")]
        pub stock_keeping_unit_selection_mode: String,
        #[serde(rename = "Score")]
        pub score: Option<i32>,
        #[serde(rename = "LinkId")]
        pub link_id: Option<String>,
        #[serde(rename = "HasChildren")]
        pub has_children: Option<bool>,
    }

    impl Category {
        pub fn new(
            id: Option<i32>,
            name: String,
            father_category_id: Option<i32>,
            title: String,
            description: String,
            keywords: String,
            is_active: bool,
            lomadee_campaign_code: Option<String>,
            ad_words_remarketing_code: Option<String>,
            show_in_store_front: bool,
            show_brand_filter: bool,
            active_store_front_link: bool,
            global_category_id: Option<i32>,
            stock_keeping_unit_selection_mode: String,
            score: Option<i32>,
            link_id: Option<String>,
            has_children: Option<bool>,
        ) -> Category {
            Category {
                id,
                name,
                father_category_id,
                title,
                description,
                keywords,
                is_active,
                lomadee_campaign_code,
                ad_words_remarketing_code,
                show_in_store_front,
                show_brand_filter,
                active_store_front_link,
                global_category_id,
                stock_keeping_unit_selection_mode,
                score,
                link_id,
                has_children,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    pub struct CategoryTree {
        pub id: i32,
        pub name: String,
        #[serde(rename = "hasChildren")]
        pub has_children: bool,
        pub url: Option<String>,
        #[serde(rename = "Title")]
        pub title: Option<String>,
        #[serde(rename = "MetaTagDescription")]
        pub meta_tag_description: Option<String>,
        pub children: Option<Vec<CategoryTree>>,
    }

    impl CategoryTree {
        pub fn new(
            id: i32,
            name: String,
            has_children: bool,
            url: Option<String>,
            title: Option<String>,
            meta_tag_description: Option<String>,
            children: Option<Vec<CategoryTree>>,
        ) -> CategoryTree {
            CategoryTree {
                id,
                name,
                has_children,
                url,
                title,
                meta_tag_description,
                children,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    pub struct Brand {
        #[serde(rename = "Id")]
        pub id: Option<i32>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(rename = "Text")]
        pub text: Option<String>,
        #[serde(rename = "Keywords")]
        pub keywords: Option<String>,
        #[serde(rename = "SiteTitle")]
        pub site_title: Option<String>,
        #[serde(rename = "Active")]
        pub active: bool,
        #[serde(rename = "MenuHome")]
        pub menu_home: Option<String>,
        #[serde(rename = "AdWordsRemarketingCode")]
        pub ad_words_remarketing_code: Option<String>,
        #[serde(rename = "LomadeeCampaignCode")]
        pub lomadee_campaign_code: Option<String>,
        #[serde(rename = "Score")]
        pub score: Option<i32>,
    }

    impl Brand {
        pub fn new(
            id: Option<i32>,
            name: String,
            text: Option<String>,
            keywords: Option<String>,
            site_title: Option<String>,
            active: bool,
            menu_home: Option<String>,
            ad_words_remarketing_code: Option<String>,
            lomadee_campaign_code: Option<String>,
            score: Option<i32>,
        ) -> Brand {
            Brand {
                id,
                name,
                text,
                keywords,
                site_title,
                active,
                menu_home,
                ad_words_remarketing_code,
                lomadee_campaign_code,
                score,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    pub struct BrandList {
        pub id: i32,
        pub name: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub title: Option<String>,
        #[serde(rename = "metaTagDescription")]
        pub meta_tag_description: Option<String>,
        #[serde(rename = "imageUrl")]
        pub image_url: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    pub struct Group {
        #[serde(rename = "Id")]
        pub id: Option<i32>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(rename = "CategoryId")]
        pub category_id: Option<i32>,
        #[serde(rename = "Position")]
        pub position: Option<i32>,
    }

    impl Group {
        pub fn new(
            id: Option<i32>,
            name: String,
            category_id: Option<i32>,
            position: Option<i32>,
        ) -> Group {
            Group { 
                id,
                name,
                category_id,
                position,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct SpecificationList {
        pub name: String,
        pub category_id: i32,
        pub field_id: i32,
        pub is_active: bool,
        pub is_stock_keeping_unit: bool,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    pub struct Specification {
        #[serde(rename = "Id")]
        pub id: Option<i32>,
        #[serde(rename = "FieldTypeId")]
        pub field_type_id: i32,
        #[serde(rename = "CategoryId")]
        pub category_id: Option<i32>,
        #[serde(rename = "FieldGroupId")]
        pub field_group_id: i32,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Position")]
        pub position: Option<i32>,
        #[serde(rename = "IsFilter")]
        pub is_filter: Option<bool>,
        #[serde(rename = "IsRequired")]
        pub is_required: Option<bool>,
        #[serde(rename = "IsOnProductDetails")]
        pub is_on_product_details: Option<bool>,
        #[serde(rename = "IsStockKeepingUnit")]
        pub is_stock_keeping_unit: Option<bool>,
        #[serde(rename = "IsWizard")]
        pub is_wizard: Option<bool>,
        #[serde(rename = "IsActive")]
        pub is_active: Option<bool>,
        #[serde(rename = "IsTopMenuLinkActive")]
        pub is_top_menu_link_active: Option<bool>,
        #[serde(rename = "DefaultValue")]
        pub default_value: Option<String>,
    }

    impl Specification {
        pub fn new(
            id: Option<i32>,
            field_type_id: i32,
            category_id: Option<i32>,
            field_group_id: i32,
            name: String,
            description: Option<String>,
            position: Option<i32>,
            is_filter: Option<bool>,
            is_required: Option<bool>,
            is_on_product_details: Option<bool>,
            is_stock_keeping_unit: Option<bool>,
            is_wizard: Option<bool>,
            is_active: Option<bool>,
            is_top_menu_link_active: Option<bool>,
            default_value: Option<String>,
        ) -> Specification {
            Specification {
                id,
                field_type_id,
                category_id,
                field_group_id,
                name,
                description,
                position,
                is_filter,
                is_required,
                is_on_product_details,
                is_stock_keeping_unit,
                is_wizard,
                is_active,
                is_top_menu_link_active,
                default_value,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
    pub struct SpecificationValue {
        #[serde(rename = "FieldValueId")]
        pub field_value_id: Option<i32>,
        #[serde(rename = "FieldId")]
        pub field_id: i32,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(rename = "Text")]
        pub text: Option<String>,
        #[serde(rename = "IsActive")]
        pub is_active: Option<bool>,
        #[serde(rename = "Position")]
        pub position: Option<i32>,
    }

    impl SpecificationValue {
        pub fn new(
            field_value_id: Option<i32>,
            field_id: i32,
            name: String,
            text: Option<String>,
            is_active: Option<bool>,
            position: Option<i32>,
        ) -> SpecificationValue {
            SpecificationValue {
                field_value_id,
                field_id,
                name,
                text,
                is_active,
                position,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct FieldValueList {
        pub field_value_id: i32,
        pub value: String,
        pub is_active: bool,
        pub position: i32,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct Product {
        pub id: Option<i32>,
        pub name: String,
        pub department_id: Option<i32>,
        pub category_id: i32,
        pub brand_id: i32,
        pub link_id: Option<String>,
        pub ref_id: Option<String>,
        pub is_visible: Option<bool>,
        pub description: Option<String>,
        pub description_short: Option<String>,
        pub release_date: Option<String>,
        pub key_words: Option<String>,
        pub title: Option<String>,
        pub is_active: Option<bool>,
        pub tax_code: Option<String>,
        pub meta_tag_description: Option<String>,
        pub supplier_id: Option<i32>,
        pub show_without_stock: Option<bool>,
        pub ad_words_remarketing_code: Option<String>,
        pub lomadee_campaign_code: Option<String>,
        pub score: Option<i32>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct Sku {
        pub id: Option<i32>,
        pub product_id: i32,
        pub is_active: Option<bool>,
        pub name: String,
        pub ref_id: Option<String>,
        pub packaged_height: f32,
        pub packaged_length: f32,
        pub packaged_width: f32,
        pub packaged_weight_kg: f32,
        pub height: Option<f32>,
        pub length: Option<f32>,
        pub width: Option<f32>,
        pub weight_kg: Option<f32>,
        pub cubic_weight: Option<f32>,
        pub is_kit: Option<bool>,
        pub creation_date: Option<String>,
        pub reward_value: Option<f32>,
        pub estimated_date_arrival: Option<String>,
        pub manufacturer_code: Option<String>,
        pub commercial_condition_id: Option<i32>,
        pub measurement_unit: Option<String>,
        pub unit_multiplier: Option<f32>,
        pub modal_type: Option<String>,
        pub kit_itens_sell_apart: Option<bool>,
        pub activate_if_possible: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct ProductSpecification {
        pub id: Option<i32>,
        pub product_id: i32,
        pub field_id: i32,
        pub field_value_id: Option<i32>,
        pub text: Option<String>,
    }
    
    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct SkuSpecAssignment {
        pub id: Option<i32>,
        pub sku_id: i32,
        pub field_id: i32,
        pub field_value_id: Option<i32>,
        pub text: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct SkuFile {
        pub id: Option<i32>,
        pub sku_id: i32,
        pub archive_id: Option<i32>,
        pub name: Option<String>,
        pub is_main: Option<bool>,
        pub label: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Price {
        pub sku_id: i32,
        pub markup: Option<f32>,
        pub list_price: Option<f32>,
        pub base_price: Option<f32>,
        pub cost_price: Option<f32>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct PriceGet {
        pub item_id: String,
        pub markup: Option<f32>,
        pub list_price: Option<f32>,
        pub base_price: Option<f32>,
        pub cost_price: Option<f32>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Inventory {
        pub warehouse_id: String,
        pub sku_id: i32,
        pub unlimited_quantity: bool,
        pub date_utc_on_balance_system: Option<String>,
        pub quantity: i32,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct InventoryList {
        pub sku_id: String,
        pub balance: Vec<Balance>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Balance {
        pub warehouse_id: String,
        pub warehouse_name: String,
        pub total_quantity: i32,
        pub reserved_quantity: i32,
        pub has_unlimited_quantity: bool,
        pub time_to_refill: Option<String>,
        pub date_of_supply_utc: Option<String>,
    }


    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct SkuAndContext {
        pub id: i32,
        pub product_id: i32,
        pub name_complete: String,
        pub complement_name: Option<String>,
        pub product_name: String,
        pub product_description: String,
        pub product_ref_id: String,
        pub tax_code: Option<String>,
        pub sku_name: String,
        pub is_active: bool,
        pub is_transported: bool,
        pub is_inventoried: bool,
        pub is_gift_card_recharge: bool,
        pub image_url: Option<String>,
        pub detail_url: String,
        #[serde(rename="CSCIdentification")]
        pub csc_identification: Option<String>,
        pub brand_id: String,
        pub brand_name: String,
        pub is_brand_active: bool,
        pub dimension: Dimension,
        pub real_dimension: RealDimension,
        pub manufacturer_code: Option<String>,
        pub is_kit: bool,
        // pub kit_items: Option<_>,
        // pub services: Option<_>,
        // pub categories: Option<_>,
        // pub categories_full_path: Vec<String>,
        // pub attachments: Option<_>,
        // pub collections: Option<_>,
        pub sku_sellers: Vec<SkuSeller>,
        pub sales_channels: Vec<i32>,
        pub images: Option<Vec<Image>>,
        // pub videos: Option<_>,
        pub sku_specifications: Option<Vec<SkuSpecification>>,
        // pub product_specifications: Option<_>,
        // pub product_clusters_ids: Option<String>,
        // pub positions_in_clusters: Option<_>,
        // pub product_cluster_names: Option<_>,
        // pub product_cluster_highlights: Option<_>,
        pub product_category_ids: String,
        pub is_direct_category_active: bool,
        // pub product_global_category_id: Option<_>,
        pub product_categories: serde_json::Value,
        pub commercial_condition_id: i32,
        pub reward_value: f32,
        pub alternate_ids: AlternateIds,
        pub alternate_id_values: Vec<String>,
        pub estimated_date_arrival: Option<String>,
        pub measurement_unit: String,
        pub unit_multiplier: f32,
        pub information_source: Option<String>,
        pub modal_type: Option<String>,
        pub key_words: Option<String>,
        pub release_date: Option<String>,
        pub product_is_visible: bool,
        pub show_if_not_available: bool,
        pub is_product_active: bool,
        pub _product_final_score: i32,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    pub struct Dimension {
        pub cubicweight: f32,
        pub height: f32,
        pub length: f32,
        pub weight: f32,
        pub width: f32,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct RealDimension {
        pub real_cubic_weight: f32,
        pub real_height: f32,
        pub real_length: f32,
        pub real_weight: f32,
        pub real_width: f32,

    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct SkuSeller {
        pub seller_id: String,
        pub stock_keeping_unit_id: i32,
        pub seller_stock_keeping_unit_id: String,
        pub is_active: bool,
        pub freight_commission_percentage: f32,
        pub product_commission_percentage: f32,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct Image {
        pub image_url: String,
        pub image_name: String,
        pub file_id: i32,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct SkuSpecification {
        pub field_id: i32,
        pub field_name: String,
        pub field_value_ids: Vec<i32>,
        pub field_values: Vec<String>,
        pub is_filter: bool,
        pub field_group_id: i32,
        pub field_group_name: String,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    pub struct ProductCategories {
        #[serde(rename="3")]
        pub three: String,
        #[serde(rename="2")]
        pub two: String,
        #[serde(rename="1")]
        pub one: String,

    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct AlternateIds {
        pub ref_id: String,
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Brand;
    use crate::model::Category;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn create_new_brand() {
        let brand: Brand = Brand {
            id: None,
            name: "Nike".to_string(),
            text: Some("Nike text".to_string()),
            keywords: Some("Nike keywords".to_string()),
            site_title: Some("Nike site title".to_string()),
            active: true,
            ad_words_remarketing_code: None,
            lomadee_campaign_code: None,
            menu_home: None,
            score: None,
        };

        assert_eq!(
            brand,
            Brand::new(
                None,
                "Nike".to_string(),
                Some("Nike text".to_string()),
                Some("Nike keywords".to_string()),
                Some("Nike site title".to_string()),
                true,
                None,
                None,
                None,
                None,
            ),
        );
    }

    #[test]
    fn create_new_category() {
        let category: Category = Category {
            id: None,
            name: "Mens Clothing".to_string(),
            father_category_id: None,
            title: "Mens Clothing".to_string(),
            description: "Mens Clothing".to_string(),
            keywords: "Mens Clothing".to_string(),
            is_active: true,
            lomadee_campaign_code: None,
            ad_words_remarketing_code: None,
            show_in_store_front: true,
            show_brand_filter: false,
            active_store_front_link: false,
            global_category_id: None,
            stock_keeping_unit_selection_mode: "SPECIFICATION".to_string(),
            score: None,
            link_id: None,
            has_children: None,
        };

        assert_eq!(
            category,
            Category::new(
                None,
                "Mens Clothing".to_string(),
                None,
                "Mens Clothing".to_string(),
                "Mens Clothing".to_string(),
                "Mens Clothing".to_string(),
                true,
                None,
                None,
                true,
                false,
                false,
                None,
                "SPECIFICATION".to_string(),
                None,
                None,
                None,
            ),
        );
    }
}
