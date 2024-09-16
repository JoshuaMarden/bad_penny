// ╔═════════════════════════════ ◈{ Imports }◈ ══════════════════════════════╗


use tokio;
use reqwest::cookie::Jar;
use std::sync::Arc;
use reqwest::Client;
use reqwest::Url;
use reqwest::header::{USER_AGENT, ACCEPT};
use reqwest::Error;
use reqwest::get;
use sqlx::postgres::PgPoolOptions;
use std::env;
use futures::future::join_all;
use std::collections::HashMap;
use std::fs;
use serde::Deserialize;
use serde_yaml;

use crate::config; // gets the module
use crate::config::Config; // gets the struct `Config`


// ╔═════════════════════════ ◈{ Import  Globals }◈ ══════════════════════════╗

pub fn use_config_data() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Load the configuration
    let config = config::load_config()?;

    // Step 2: Use the `ons_website` field
    let ons_website = &config.ons_website;

    // Step 3: Use the data_sources HashMap in your logic
    let api_urls = &config.data_sources;

    // Example of accessing a specific API URL:
    if let Some(cpi_api_url) = api_urls.get("ons_cpi_api") {
        // Use `cpi_api_url` in your logic, for example making requests
    }

    // Step 4: Use the browser emulation details as needed
    let user_agent = &config.browser_emulation_details.user_agent;
    let accept_response = &config.browser_emulation_details.accept_response;


    Ok(())
}

// ╔═════════════════════════ ◈{ Fetcth  Data }◈ ══════════════════════════╗


/// Send an HTTP request with spoofed headers and cookie support.
///
/// Function is important for emulating a browser so we are not blocked
/// from accessing ONS.
///
/// # Arguments
///
/// * `config` - The configuration object with headers and URLs.
///
/// # Returns
///
/// A `Client` with cookies enabled for reuse in other functions.
pub async fn send_spoofed_request(config: &Config) -> Result<Client, Box<dyn std::error::Error>> {
    // Step 1: Set up the HTTP client with cookie support
    let cookie_jar = Arc::new(Jar::default());
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(cookie_jar.clone())
        .build()?;

    // Step 2: Use the URL from the passed-in config
    let url = config.ons_website.parse::<Url>()?;

    // Step 3: Use headers from the config (User-Agent and Accept)
    let response = client
        .get(url.clone())
        .header("User-Agent", &config.browser_emulation_details.user_agent)
        .header("Accept", &config.browser_emulation_details.accept_response)
        .header("Referer", "https://www.ons.gov.uk/")
        .header("Connection", "keep-alive")
        .header("DNT", "1")
        .header("Upgrade-Insecure-Requests", "1")
        .header("Sec-Fetch-Site", "none")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-User", "?1")
        .header("Sec-Fetch-Dest", "document")
        .body("Some additional body content to simulate real traffic") 
        .send()
        .await?;

    // Handle the response (check for success)
    if response.status().is_success() {
        println!("Initial spoofed request successful.");
    } else {
        println!("Initial spoofed request failed.");
    }

    // Return the client with cookie support
    Ok(client)
}


/// Fetches data from a given API URL asynchronously using the provided `Client`.
///
/// Sends an HTTP GET request to the provided URL and returns 
/// the response body as a `String`.
///
/// # Arguments
///
/// * `client` - The `reqwest::Client` with cookies and headers configured.
/// * `api_url` - A string slice containing the URL to fetch data from.
///
/// # Returns
///
/// Returns `Ok(String)` containing the response body if the request is successful;
/// `Err(reqwest::Error)` if the request fails.
pub async fn fetch_data(client: &Client, api_url: &str) -> Result<String, reqwest::Error> {
    let response = client.get(api_url).send().await?;
    let response_text = response.text().await?;
    
    Ok(response_text)
}


/// Fetches all the data from API URLs concurrently using spoofed headers and cookies.
///
/// Takes a `Config` object, extracts all the API URLs from the `data_sources` HashMap,
/// and uses the `send_spoofed_request` function to fetch the data with cookies and headers.
/// 
/// # Arguments
/// 
/// * `config` - Ref to a `Config` object holding API URLs.
/// 
/// # Returns
/// 
/// `Ok(())` if all requests are successful; error if any fail.
pub async fn fetch_all_data(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Get the client with cookies and spoofing configured
    let client = send_spoofed_request(config).await?;

    // Step 2: Extract API URLs from the `data_sources` HashMap
    let api_urls = &config.data_sources; 

    // Step 3: Create futures for each URL and collect them
    let fetch_futures = api_urls.values().map(|url| fetch_data(&client, url));  // Pass the client
    let results = join_all(fetch_futures).await;  // Join all futures concurrently

    // Step 4: Handle the results of each request
    for result in results {
        match result {
            Ok(data) => println!("Received data: {}", data),
            Err(e) => println!("Error fetching data: {}", e),
        }
    }

    Ok(())
}

