use reqwest::{Client, StatusCode};
use vtex::model::SkuSpecAssignment;
use std::error::Error;
use std::fs::File;


pub async fn load_sku_specs(file_path: String, client: &Client, url: String) -> Result<(), Box<dyn Error>> {

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    for line in rdr.deserialize() {
        let record: SkuSpecAssignment = line?;
        let url_with_sku_id = url.replace("{skuId}", record.sku_id.to_string().as_str());
        println!("url: {}", url_with_sku_id);
        let payload = serde_json::to_string(&record).unwrap();
        println!("payload: {}", payload);
        let response = client
            .post(&url_with_sku_id)
            .json(&record)
            .send()
            .await?;

        println!("response.status: {}", response.status());
        match response.status() {
            StatusCode::OK => {
                let body = response.text().await?;
                // println!("body: {:?}", body);
                let result: Result<SkuSpecAssignment, serde_json::Error> = serde_json::from_str(&body);
                match result {
                    Ok(spec) => println!("spec id: {}", spec.id.unwrap()),
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