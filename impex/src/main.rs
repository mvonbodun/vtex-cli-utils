use log::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    vtex_impex::setup();
    info!("Starting data load");
    vtex_impex::run().await?;
    info!("Finished data load");
    
    Ok(())
}
