use reqwest::{Client, StatusCode};
use vtex::model::SkuFile;
use std::{error::Error, fs::File};

pub async fn load_sku_files(file_path: String, client: &Client, url: String) -> Result<(), Box<dyn Error>> {

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    for line in rdr.deserialize() {
        let record: SkuFile = line?;
        // println!("SkuFile: {:?}", record);
        let url_with_sku_id = url.replace("{skuId}", record.sku_id.to_string().as_str());

        let response = client
            .post(&url_with_sku_id)
            .json(&record)
            .send()
            .await?;

        println!("response.status: {}", response.status());
        match response.status() {
            StatusCode::OK => {
                let sku_file_json = response.text().await?;
                // println!("sku_json: {:?}", sku_json);
                let result: Result<SkuFile, serde_json::Error> = serde_json::from_str(&sku_file_json);
                match result {
                    Ok(sku_file) => println!("sku_file_id: {}", sku_file.id.unwrap()),
                    Err(e) => println!("deserialize product error: {:?}", e),
                }
            },
            _ => {
                println!("Status Code: [{:?}] Error: [{:#?}] \n record: {:?}", response.status(), response.text().await?, record);
            },
        }
    }
    Ok(())
}