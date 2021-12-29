use std::error::Error;
use std::fs::File;

use vtex::model::Brand;
use reqwest::Client;

pub async fn load_brands(file_path: String, client: &Client, url: String) -> Result<(), Box<dyn Error>> {
    
    println!("before File::open: {}", file_path);
    let input = File::open(file_path)?;
    println!("after File::open");
    let mut rdr = csv::Reader::from_reader(input);
    println!("before for loop");
    for line in rdr.deserialize() {
        let record: Brand = line?;
        println!("{:?}", record);

        let new_post: Brand = client
            .post(&url)
            .json(&record)
            .send()
            .await?
            .json()
            .await?;

        println!("before print new_post");
        println!("{:#?}", new_post);
        println!("record id: {:?}", new_post.id);
        println!("after print new_post");
    
    }
    
    Ok(())
}
