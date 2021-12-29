use std::error::Error;
use std::fs::File;
use reqwest::{Client, StatusCode};
use vtex::model::Product;
use crate::csvrecords::ProductLookup;

pub async fn load_products(file_path: String, client: &Client, url: String) -> Result<(), Box<dyn Error>> {

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);
    let out_path = "data/ProductLookup.csv";
    let mut writer = csv::Writer::from_path(out_path)?;

    for line in rdr.deserialize() {
        let record: Product = line?;
        // println!("Product: {:?}", record);

        let response = client
            .post(&url)
            .json(&record)
            .send()
            .await?;

        println!("response.status: {}", response.status());
        match response.status() {
            StatusCode::OK => {
                // println!("response.text() {:#?}", response.text().await?);
                let result = response.json::<Product>().await;
                match result {
                    Ok(product) => { 
                        println!("product_id: {}", product.id.unwrap());
                        let sku_lookup = ProductLookup {
                            part_number: product.ref_id.unwrap(),
                            product_id: product.id.unwrap(),
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