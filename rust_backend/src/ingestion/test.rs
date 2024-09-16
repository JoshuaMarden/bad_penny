use fantoccini::{Client, Locator};

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let mut client = Client::new("http://localhost:4444").await?;

    // Open the ONS website
    client.goto("https://www.ons.gov.uk").await?;

    // Take a screenshot to verify page load
    let screenshot = client.screenshot().await?;
    std::fs::write("ons_test.png", &screenshot)?;

    client.close().await?;
    Ok(())
}