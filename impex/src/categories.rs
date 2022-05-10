use anyhow::{Context, Result};
use log::*;
use std::collections::HashMap;
use std::fs::File;

use reqwest::{Client, StatusCode};
use vtex::model::Category;

pub async fn load_categories(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
) -> Result<()> {
    info!("Begin loading categories");
    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/category"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);

    let input =
        File::open(&file_path).with_context(|| format!("could not read file `{}`", &file_path))?;
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
        if !parent_unique_identifier.is_empty() {
            let cat_id = category_ids.get(&parent_unique_identifier);
            match cat_id {
                Some(v) => father_category_id = Some(*v),
                None => father_category_id = None,
            }
        }

        debug!("father_category_id: {:?}", father_category_id);

        let new_post = Category {
            id: record.id,
            unique_identifier: record.unique_identifier.clone(),
            name: record.name.to_string(),
            father_category_id,
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
            has_children: record.has_children,
        };

        let response = client.post(&url).json(&new_post).send().await?;

        match response.status() {
            StatusCode::OK => {
                let result = response.json::<Category>().await;
                match result {
                    Ok(category) => {
                        category_ids.insert(
                            record.unique_identifier.unwrap().clone(),
                            category.id.unwrap(),
                        );
                        info!(
                            "category id: {}: response: {:?}",
                            category.id.unwrap(),
                            StatusCode::OK
                        );
                    }
                    Err(e) => error!("deserialize category error: {:?}", e),
                }
            }
            _ => {
                info!(
                    "Status Code: [{:?}] Error: [{:#?}] \n record: {:?}",
                    response.status(),
                    response.text().await?,
                    new_post
                );
            }
        }

        debug!("{:#?}", new_post);
    }
    debug!("HashMap size: {}", category_ids.len());
    info!("Finished loading categories");

    Ok(())
}

pub async fn update_categories(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
) -> Result<()> {
    info!("Begin updating categories");
    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/category/{categoryId}"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);

    let input =
        File::open(&file_path).with_context(|| format!("could not read file `{}`", &file_path))?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut category_recs: Vec<Category> = Vec::new();
    info!("Start: Reading input file to ensure values can be parsed");
    let mut e = 0;
    for line in rdr.deserialize() {
        match line {
            Ok(record) => {
                let category_rec: Category = record;
                category_recs.push(category_rec);
                e += 1;
            }
            Err(err) => {
                error!("Error parsing row: {:?}", err);
            }
        }
    }
    info!("Finished: Reading input file");
    info!(
        "Records successfully read: {}. Records not read (errors): {}",
        category_recs.len(),
        e
    );

    // Now process the category_recs
    for line in category_recs {
        debug!("{:?}", line);

        let new_post = Category {
            id: line.id,
            unique_identifier: line.unique_identifier.clone(),
            name: line.name,
            father_category_id: line.father_category_id,
            parent_unique_identifier: line.parent_unique_identifier,
            title: line.title,
            description: line.description,
            keywords: line.keywords,
            is_active: line.is_active,
            lomadee_campaign_code: line.lomadee_campaign_code,
            // Store the category unique_identifier in ad_words_remarketing_code field - deprecated
            ad_words_remarketing_code: line.unique_identifier.clone(),
            show_in_store_front: line.show_in_store_front,
            show_brand_filter: line.show_brand_filter,
            active_store_front_link: line.active_store_front_link,
            global_category_id: line.global_category_id,
            stock_keeping_unit_selection_mode: line.stock_keeping_unit_selection_mode,
            score: line.score,
            link_id: line.link_id,
            has_children: line.has_children,
        };

        let url_with_category_id =
            url.replace("{categoryId}", line.id.unwrap().to_string().as_str());

        let response = client
            .put(&url_with_category_id)
            .json(&new_post)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let result = response.json::<Category>().await;
                match result {
                    Ok(category) => {
                        info!(
                            "category id: {}: response: {:?}",
                            category.id.unwrap(),
                            StatusCode::OK
                        );
                    }
                    Err(e) => error!("deserialize category error: {:?}", e),
                }
            }
            _ => {
                info!(
                    "Status Code: [{:?}] Error: [{:#?}] \n record: {:?}",
                    response.status(),
                    response.text().await?,
                    new_post
                );
            }
        }

        debug!("{:#?}", new_post);
    }
    info!("Finished updating categories");

    Ok(())
}
