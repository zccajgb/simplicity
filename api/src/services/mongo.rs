use anyhow::Result;
use mongodb::{options::ClientOptions, Client};

pub async fn get_client() -> Result<Client> {
    let mongourl = std::env::var("MONGO_URL").expect("MONGO_URL must be set");
    let client_options = ClientOptions::parse(mongourl).await?;
    // client_options.app_name = Some("".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client)
}
