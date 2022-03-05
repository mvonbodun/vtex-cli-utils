use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use log::*;

use vtex::model::Category;
use reqwest::{Client, StatusCode};

pub async fn load_categories(file_path: String, client: &Client, account_name: String, environment: String) -> Result<(), Box<dyn Error>> {

    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/category"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);
    let mut category_ids: HashMap<String, i32> = HashMap::new();

    for line in rdr.deserialize() {
        let record: Category = line?;
        debug!("{:?}", record);

        let parent_unique_identifier;
        match record.parent_unique_identifier {
            Some(v) => parent_unique_identifier = v,
            None => parent_unique_identifier = "".to_string(),
        }

        let mut father_category_id: Option<i32> = None; 
        if parent_unique_identifier.len() > 0 {
            let cat_id = category_ids.get(&parent_unique_identifier);
            match cat_id {
                Some(v) => father_category_id = Some(*v),
                None => father_category_id = None,
            }
        }

        debug!("father_category_id: {:?}", father_category_id);

        let new_post = Category {
            id: None,
            unique_identifier: record.unique_identifier.clone(),
            name: record.name.to_string(),
            father_category_id: father_category_id.clone(),
            parent_unique_identifier: Some(parent_unique_identifier),
            title: record.title.to_string(),
            description: record.description.to_string(),
            keywords: record.keywords,
            is_active: record.is_active,
            lomadee_campaign_code: record.lomadee_campaign_code,
            // Store the category unique_identifier in ad_words_remarketing_code field - deprecated
            ad_words_remarketing_code: record.unique_identifier.clone(),
            show_in_store_front: record.show_in_store_front,
            show_brand_filter: record.show_brand_filter,
            active_store_front_link: record.active_store_front_link,
            global_category_id: record.global_category_id,
            stock_keeping_unit_selection_mode: record.stock_keeping_unit_selection_mode,
            score: record.score,
            link_id: record.link_id,
            has_children: record.has_children
        };
    
        println!("before let new_post");
        let response = client
            .post(&url)
            .json(&new_post)
            .send()
            .await?;

            match response.status() {
                StatusCode::OK => {
                    // println!("response.text() {:#?}", response.text().await?);
                    let result = response.json::<Category>().await;
                    match result {
                        Ok(category) => { 
                            category_ids.insert(record.unique_identifier.unwrap().clone(), category.id.unwrap().clone());
                            info!("category id: {}: response: {:?}", category.id.unwrap(), StatusCode::OK);
                        },
                        Err(e) => error!("deserialize category error: {:?}", e),
                    }
                },
                _ => {
                    println!("Status Code: [{:?}] Error: [{:#?}] \n record: {:?}", response.status(), response.text().await?, new_post);
                },
            }
    
        // let new_post: Category = response.json().await?;
    
        // info!("category: {:?}: response: {:?}", new_post.id, response.status());
        debug!("{:#?}", new_post);
        // println!("after print new_post");
    }
    debug!("HashMap size: {}", category_ids.len());

    Ok(())

}