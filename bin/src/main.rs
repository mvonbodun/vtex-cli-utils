use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    vtex_dataload::run().await?;

    Ok(())
}
