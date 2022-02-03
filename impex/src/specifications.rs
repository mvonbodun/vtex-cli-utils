use std::error::Error;
use std::fs::File;

use reqwest::Client;
use vtex::model::Specification;

pub async fn load_specifications(file_path: String, client: &Client, url: String) -> Result<(), Box<dyn Error>> {

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    for line in rdr.deserialize() {
        let record: Specification = line?;
        println!("Specification Record: {:?}", record);
        println!("Specification JSON: {:?}", serde_json::to_string_pretty(&record));

        let new_post: Specification = client
            .post(&url)
            .json(&record)
            .send()
            .await?
            .json()
            .await?;

        println!("new_post: {:#?}", new_post);
        println!("Id: {:?}", new_post.id);
        println!("after print new_post");
    }

    Ok(())
}