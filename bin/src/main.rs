use log::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    vtex_dataload::setup();
    info!("Starting data load");
    vtex_dataload::run().await?;
    info!("Finished data load");
    
    Ok(())
}
