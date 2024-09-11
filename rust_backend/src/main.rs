// ╔═════════════════════════════ ◈{ Imports }◈ ══════════════════════════════╗

use reqwest::cookie::Jar;
use std::sync::Arc;
use reqwest::Client;
use reqwest::Url;
use reqwest::header::{USER_AGENT, ACCEPT};


mod config;
mod ingestion {
    pub mod ons_api;
}

// ╔═════════════════════════ ◈{ Run  Modules }◈ ══════════════════════════╗

// Define the main entry point for the application
#[tokio::main]  // If you are using async code, add this attribute
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Load the configuration using load_config
    let config = config::load_config()?;  // Or you can use your `use_config_data` if it wraps this

    // Step 2: Pass the `Config` object to `fetch_all_data`
    ingestion::ons_api::fetch_all_data(&config).await?;  // Don't forget to await the async function

    Ok(())
}
