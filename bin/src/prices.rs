use reqwest::{Client, StatusCode};
use vtex::model::Price;
use std::error::Error;
use std::fs::File;

pub async fn load_prices(file_path: String, client: &Client, url: String) -> Result<(), Box<dyn Error>> {

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    for line in rdr.deserialize() {
        let record: Price = line?;
        // println!("SKU: {:?}", record);
        let url_with_sku_id = url.replace("{skuId}", record.sku_id.to_string().as_str());

        let response = client
            .put(&url_with_sku_id)
            .json(&record)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                    println!("response status {:?}, sku_id: {}", response.status(), record.sku_id);
            },
            _ => {
                println!("Status Code: [{:?}] Error: [{:#?}] \n record: {:?}", response.status(), response.text().await?, record);
            },
        }
    }

    Ok(())

}