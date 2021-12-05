use dotenv;
use std::{fs::File, collections::HashMap, env};
use std::error::Error;
use serde::{Deserialize, Serialize};
use reqwest::header;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    groupIdentifier: String,
    parentGroupIdentifier: String,
    topGroup: bool,
    sequence: i32,
    name: String,
    shortDescription: String,
    longDescription: String,
    thumbnail: String,
    fullImage: String,
    field1: String,
    published: i32,
    delete: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    Id: Option<i32>,
    Name: String,
    FatherCategoryId: Option<i32>,
    Title: String,
    Description: String,
    Keywords: String,
    IsActive: bool,
    LomadeeCampaignCode: Option<String>,
    AdWordsRemarketingCode: Option<String>,
    ShowInStoreFront: bool,
    ShowBrandFilter: bool,
    ActiveStoreFrontLink: bool,
    GlobalCategoryId: Option<i32>,
    StockKeepingUnitSelectionMode: String,
    Score: Option<i32>,
    LinkId: Option<String>,
    HasChildren: Option<bool>
}
  

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve the environment variables
    dotenv::dotenv().expect("Failed to read .env file");
    let vtex_api_key = env::var("VTEX_API_APPKEY").expect("Failed to parse VTEX_API_APPKEY in .env");
    let vtex_api_apptoken = env::var("VTEX_API_APPTOKEN").expect("Failed to parse VTEX_API_APPTOKEN in .env");
    let url = env::var("URL").expect("Failed to parse URL in .env");
    // Setup the HTTP client
    let mut headers = header::HeaderMap::new();
    headers.insert("X-VTEX-API-AppKey", header::HeaderValue::from_str(&vtex_api_key)?);
    headers.insert("X-VTEX-API-AppToken", header::HeaderValue::from_str(&vtex_api_apptoken)?);
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // Read data from the CSV file
    let path = "data/DeptCatalog-sorted-subset.csv";
    let input = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(input);
    let mut category_ids: HashMap<String, i32> = HashMap::new();

    for line in rdr.deserialize() {
        let record: Record = line?;
        println!("{:?}", record);

        let mut father_category_id: Option<i32> = None; 
        if !record.parentGroupIdentifier.is_empty() {
            let cat_id = category_ids.get(&record.parentGroupIdentifier);
            match cat_id {
                Some(v) => father_category_id = Some(*v),
                None => father_category_id = None,
            }
        }

        println!("father_category_id: {:?}", father_category_id);

        let new_post = Category {
            Id: None,
            Name: record.name.to_string(),
            FatherCategoryId: father_category_id.clone(),
            Title: record.name.to_string(),
            Description: record.name.to_string(),
            Keywords: record.groupIdentifier,
            IsActive: true,
            LomadeeCampaignCode: None,
            AdWordsRemarketingCode: None,
            ShowInStoreFront: true,
            ShowBrandFilter: true,
            ActiveStoreFrontLink: true,
            GlobalCategoryId: None,
            StockKeepingUnitSelectionMode: "SPECIFICATION".to_string(),
            Score: None,
            LinkId: None,
            HasChildren: None
        };
    
        println!("before let new_post");
        let new_post: Category = client
            .post(&url)
            .json(&new_post)
            .send()
            .await?
            .json()
            .await?;
    
        println!("before print new_post");
        println!("{:#?}", new_post);
        println!("after print new_post");
        category_ids.insert(new_post.Keywords.clone(), new_post.Id.unwrap().clone());
    }
    println!("HashMap size: {}", category_ids.len());

    Ok(())
}
