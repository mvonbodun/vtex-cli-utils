use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use serde::{ Deserialize };

use vtex::model::Category;
use reqwest::Client;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    group_identifier: String,
    parent_group_identifier: String,
    _top_group: bool,
    _sequence: i32,
    name: String,
    _short_description: String,
    _long_description: String,
    _thumbnail: String,
    _full_image: String,
    _field1: String,
    _published: i32,
    _delete: i32
}

pub async fn load_categories(file_path: String, client: &Client, url: String) -> Result<(), Box<dyn Error>> {

    // let input = File::open(file_path)?;
    // let mut rdr = csv::Reader::from_reader(input);

    // Read data from the CSV file
    // let path = "data/DeptCatalog-sorted-subset.csv";
    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);
    let mut category_ids: HashMap<String, i32> = HashMap::new();

    for line in rdr.deserialize() {
        let record: Record = line?;
        println!("{:?}", record);

        let mut father_category_id: Option<i32> = None; 
        if !record.parent_group_identifier.is_empty() {
            let cat_id = category_ids.get(&record.parent_group_identifier);
            match cat_id {
                Some(v) => father_category_id = Some(*v),
                None => father_category_id = None,
            }
        }

        println!("father_category_id: {:?}", father_category_id);

        let new_post = Category {
            id: None,
            name: record.name.to_string(),
            father_category_id: father_category_id.clone(),
            title: record.name.to_string(),
            description: record.name.to_string(),
            keywords: record.group_identifier,
            is_active: true,
            lomadee_campaign_code: None,
            ad_words_remarketing_code: None,
            show_in_store_front: true,
            show_brand_filter: true,
            active_store_front_link: true,
            global_category_id: None,
            stock_keeping_unit_selection_mode: "SPECIFICATION".to_string(),
            score: None,
            link_id: None,
            has_children: None
        };
    
        println!("before let new_post");
        let new_post: Category = client
            .post(&url)
            .json(&new_post)
            .send()
            .await?
            .json()
            .await?;
    
        println!("before print new_post");
        println!("{:#?}", new_post);
        println!("after print new_post");
        category_ids.insert(new_post.keywords.clone(), new_post.id.unwrap().clone());
    }
    println!("HashMap size: {}", category_ids.len());

    Ok(())

}