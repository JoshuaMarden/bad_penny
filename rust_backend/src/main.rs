use reqwest::cookie::Jar;
use std::sync::Arc;
use reqwest::Client;
use reqwest::Url;

mod ingestion {
    pub mod ons_api;  // Import the ons_api.rs file as a module
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cookie_jar = Arc::new(Jar::default());
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(cookie_jar.clone())
        .build()?;

    let url = "https://www.ons.gov.uk".parse::<Url>()?;
    
    // First request to get cookies
    let response = client.get(url.clone()).send().await?;
    println!("Status: {}", response.status());

    // If cookies are needed for subsequent requests
    let body = client.get(url).send().await?.text().await?;
    println!("{}", body);

    Ok(())
}
