// ╔═════════════════════════════ ◈{ Imports }◈ ══════════════════════════════╗


use tokio;
use std::sync::Arc;
use reqwest::Client;
use reqwest::Url;
use reqwest::header::{USER_AGENT, ACCEPT};
use reqwest::cookie::{Jar, CookieStore};
use reqwest::Error;
use reqwest::get;
use sqlx::postgres::PgPoolOptions;
use std::env;
use futures::future::join_all;
use std::collections::HashMap;
use std::fs;
use serde::Deserialize;
use serde_yaml;
use hyper::Uri;

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
    // Step 1: Set up the cookie jar and client
    let cookie_jar = Arc::new(Jar::default());

    // Use rustls for TLS
    let client = Client::builder()
        .cookie_provider(cookie_jar.clone())  // Attach the cookie jar
        .use_rustls_tls()  // Ensure rustls is used for TLS connections
        .build()?;

    // Step 2: Use the URL from the config
    let url = config.ons_website.parse::<Url>()?;

    // Step 3: Send the request with headers
    let response = client
        .get(url.clone())
        .header("User-Agent", &config.browser_emulation_details.user_agent)
        .header("Accept", &config.browser_emulation_details.accept_response)
        .header("Referer", "https://www.ons.gov.uk/")
        .header("Connection", "keep-alive")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Cache-Control", "no-cache")
        .header("Pragma", "no-cache")
        .header("Upgrade-Insecure-Requests", "1")
        .header("DNT", "1")
        .header("Sec-Fetch-Site", "none")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-User", "?1")
        .header("Sec-Fetch-Dest", "document")
        .header("Sec-Ch-Ua", "\"Google Chrome\";v=\"91\", \"Chromium\";v=\"91\", \";Not A Brand\";v=\"99\"")
        .header("Sec-Ch-Ua-Mobile", "?0")
        .header("Sec-Ch-Ua-Platform", "\"Windows\"")
        .header("Sec-Ch-UA-Platform-Version", "10.0.0")
        .header("Sec-Ch-UA-Arch", "\"x86\"")
        .header("Sec-Ch-UA-Full-Version-List", "\"91.0.4472.114\"")
        .header(
            "Cookie",
            "__cf_bm=sU0O_GGme3twy3BIhP4C4c9E8p1NNUPNNIjXK76kJfI-1726505917-1.0.1.1-WO2BaXduG8naVJLlCN9Hn0dzzQHkNpyhyvpsa35jvz.zjd4_5mebZYZCrkt3cMm3UaHd2N6Okwg4pfIGK7b0SA;\
             _cfuvid=pzY4Y0XKZWPgpysi6mN9ghlBV4L7BFRUHkczH0Y2e.o-1726494065566-0.0.1.1-604800000;\
             ab_test=%7B%22dp-frontend-search-controller%22%3A%7B%22new%22%3A%222024-09-06T13%3A40%3A11%22%2C%22old%22%3A%222024-09-05T13%3A40%3A11%22%7D%7D;\
             cf_clearance=8iWss0i6Ii2NwSrlqdv4ZqhCAIlxe9StcLzCnqXgAKI-1726579698-1.2.1.1-h4vmYwi3Lvn8xxqogyjlfnPK7Yr7qqVb.8roqIwt.hXDLCwj2_QWcFF7LyNgrdyVx9w3g9TVFk2QPstnsj_ik8ghQMNKeDJow1cEe4lboku9u4ubryMK5"
        )
        .body("Some additional body content to simulate real traffic")
        .send()
        .await?;

    // Check the response status
    if response.status().is_success() {
        println!("Initial spoofed request successful.");
    } else {
        println!("Initial spoofed request failed: {:?}", response.status());
    }

    // Step 4: Inspect and print cookies after the request
    if let Some(cookies) = cookie_jar.cookies(&url) {  // Use &url instead of &uri
        println!("Stored cookies: {:?}", cookies);
    } else {
        println!("No cookies stored.");
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

