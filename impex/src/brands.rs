use futures::{stream, StreamExt};
use log::*;
use std::fs::File;
use std::{collections::HashSet, error::Error};

use reqwest::Client;
use vtex::model::{Brand, Product};

pub fn gen_brand_file(file_path: String, product_file: String) -> Result<(), Box<dyn Error>> {
    let in_file = File::open(&product_file).unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = file_path;
    let mut writer = csv::Writer::from_path(out_path)?;

    let mut brand_set: HashSet<String> = HashSet::new();

    // Loop through the product file to pull out the brands and store in a HashSet
    for line in reader.deserialize() {
        let record: Product = line?;
        debug!("product record: {:?}", record);

        if !brand_set.contains(&record.brand_name.as_ref().unwrap().clone()) {
            brand_set.insert(record.brand_name.unwrap());
        }
    }

    // Loop through the HashSet and create the Brand record
    let mut x = 0;
    for brand in brand_set {
        let b = Brand {
            id: None,
            name: brand.clone(),
            text: Some(brand.clone()),
            keywords: Some(brand.clone()),
            site_title: None,
            active: true,
            // Set to false instead of None or the admin doesn't work properly
            menu_home: Some("false".to_string()),
            ad_words_remarketing_code: None,
            lomadee_campaign_code: None,
            score: None,
        };
        // Write the record
        writer.serialize(b)?;
        x += 1;
    }
    // Flush the records
    writer.flush()?;
    info!("Wrote {} brand records", x);

    Ok(())
}

pub async fn load_brands(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
) -> Result<(), Box<dyn Error>> {
    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/brand"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);
    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut brand_recs: Vec<Brand> = Vec::new();

    for line in rdr.deserialize() {
        let record: Brand = line?;
        brand_recs.push(record);
    }
    info!("brand records: {:?}", brand_recs.len());

    let bodies = stream::iter(brand_recs)
        .map(|record| {
            let client = &client;
            let url = &url;
            async move {
                let response = client.post(url).json(&record).send().await?;

                info!("brand: {:?}: response: {:?}", record.id, response.status());

                response.json::<Brand>().await
            }
        })
        .buffer_unordered(concurrent_requests);
    bodies
        .for_each(|b| async {
            match b {
                Ok(b) => info!("output: {:?}", b),
                Err(e) => error!("error: {:?}", e),
            }
        })
        .await;

    info!("Finished loading brands");
    Ok(())
}
