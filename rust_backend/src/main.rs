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
    // Example: Call a function from `ons_api`
    ingestion::ons_api::use_config_data()?;

    Ok(())
}
