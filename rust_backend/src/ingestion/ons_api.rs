// ╔═════════════════════════════ ◈{ Imports }◈ ══════════════════════════════╗


use tokio;
use reqwest::Error;
use reqwest::get;
use sqlx::postgres::PgPoolOptions;
use std::env;
use futures::future::join_all;
use serde::Deserialize;
use serde_yaml;
use std::fs;

// ╔═════════════════════════ ◈{ Import  Globals }◈ ══════════════════════════╗


#[derive(Debug, Deserialize)]
pub struct Config {
    /// Obj to hold config data
    pub data_sources: DataSources,
}

#[derive(Debug, Deserialize)]
pub struct DataSources {
    /// Obj to hold API URLs
    pub ons_cpi_api: String,
    pub ons_nominal_petrol_api: String,
    pub ons_nominal_diesel_api: String,
    pub ons_cpih_gas_api: String,
    pub ons_cpih_electricity_api: String,
}

impl DataSources {
    /// Collects all API URLs into a vector for looping.
    ///
    /// # Return 
    /// 
    /// Returns a `Vec<&String>`
    pub fn get_all_urls(&self) -> Vec<&String> {
        vec![
            &self.ons_cpi_api,
            &self.ons_nominal_petrol_api,
            &self.ons_nominal_diesel_api,
            &self.ons_cpih_gas_api,
            &self.ons_cpih_electricity_api,
        ]
    }
}

/// Loads globals from YAML into a `Config` object.
/// Reads `../global_config.yml` from project root.
///
/// # Return
/// 
/// Returns `Ok(Config)` if successful, otherwise `Err`.
pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_yaml = fs::read_to_string("../global_config.yml")?;
    let config: Config = serde_yaml::from_str(&config_yaml)?;
    Ok(config)
}

/// Fetches data from a given API URL asynchronously.
///
/// Sends an HTTP GET request to the provided URL and returns 
/// the response body as a `String`.
///
/// # Arguments
///
/// * `api_url` - A string *slice* containing the URL to fetch data from.
///
/// # Returns
///
/// Returns `Ok(String)` containing the response body if the request is successful;
/// `Err(reqwest::Error)` if the request fails.
pub async fn fetch_data(api_url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(api_url).await?;
    let response_text = response.text().await?;
    
    Ok(response_text)
}

/// Fetches data from all API URLs in the configuration concurrently.
/// 
/// Takes a `Config` obj, extracts all the API URLs from it, 
/// uses asynchronous execution to fetch data from each URL concurrently. 
/// It uses `futures::future::join_all` to execute multiple HTTP requests 
/// in parallel.
/// 
/// # Arguments
/// 
/// * `config` - Ref to a `Config` object holding API URLs.
/// 
/// # Returns
/// 
/// `Ok(())` if all requests are successful; error if any fail. 
pub async fn fetch_all_data(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    
    let api_urls = config.data_sources.get_all_urls();

    let fetch_futures = api_urls.iter().map(|url| fetch_data(url));
    let results = futures::future::join_all(fetch_futures).await;

    for result in results {
        match result {
            Ok(data) => println!("Received data: {}", data),
            Err(e) => println!("Error fetching data: {}", e),
        }
    }

    Ok(())
}
