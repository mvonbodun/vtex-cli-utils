use std::error::Error;
use std::fs::File;

use reqwest::Client;
use vtex::model::Group;

pub async fn load_groups(file_path: String, client: &Client, url: String) -> Result<(), Box<dyn Error>> {

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    for line in rdr.deserialize() {
        let record: Group = line?;
        println!("{:?}", record);

        let new_post: Group = client
            .post(&url)
            .json(&record)
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", new_post);
        println!("{:?}", new_post.id);
        println!("after print new_post");
    }

    Ok(())
}