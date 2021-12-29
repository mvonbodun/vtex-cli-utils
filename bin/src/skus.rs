use reqwest::{Client, StatusCode};
use vtex::model::Sku;

use std::{error::Error, fs::File};

use crate::csvrecords::SkuLookup;

pub async fn load_skus(file_path: String, client: &Client, url: String) -> Result<(), Box<dyn Error>> {

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);
    let out_path = "data/SkuLookup.csv";
    let mut writer = csv::Writer::from_path(out_path)?;

    for line in rdr.deserialize() {
        let record: Sku = line?;
        // println!("SKU: {:?}", record);

        let response = client
            .post(&url)
            .json(&record)
            .send()
            .await?;

        println!("response.status: {}", response.status());
        match response.status() {
            StatusCode::OK => {
                let sku_json = response.text().await?;
                // println!("sku_json: {:?}", sku_json);
                let result: Result<Sku, serde_json::Error> = serde_json::from_str(&sku_json);
                match result {
                    Ok(sku) => {
                        println!("sku_id: {}", sku.id.unwrap());
                        let sku_lookup = SkuLookup {
                            part_number: sku.ref_id.unwrap(),
                            sku_id: sku.id.unwrap(),
                        };
                        writer.serialize(&sku_lookup)?;
                    },
                    Err(e) => println!("deserialize product error: {:?}", e),
                }
            },
            _ => {
                println!("Status Code: [{:?}] Error: [{:#?}] \n record: {:?}", response.status(), response.text().await?, record);
            },
        }
    }
    // Flush the writer
    writer.flush()?;

    Ok(())
}