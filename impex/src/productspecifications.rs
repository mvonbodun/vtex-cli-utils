use reqwest::{Client, StatusCode};
use vtex::model::ProductSpecification;
use std::{error::Error, fs::File};


pub async fn load_product_specs(file_path: String, client: &Client, url: String) -> Result<(), Box<dyn Error>> {

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    for line in rdr.deserialize() {
        let record: ProductSpecification = line?;

        let response = client
            .post(&url)
            .json(&record)
            .send()
            .await?;

        println!("response.status: {}", response.status());
        match response.status() {
            StatusCode::OK => {
                let body = response.text().await?;
                // println!("body: {:?}", body);
                let result: Result<ProductSpecification, serde_json::Error> = serde_json::from_str(&body);
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