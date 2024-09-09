// ╔═════════════════════════════ ◈{ Imports }◈ ══════════════════════════════╗


use tokio;
use reqwest;
use sqlx::postgres::PgPoolOptions;
use std::env;

// ╔═════════════════════════ ◈{ Import  Globals }◈ ══════════════════════════╗

use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub data_sources: DataSources,
}

#[derive(Debug, Deserialize)]
pub struct DataSources {
    pub api_url: String,
    pub other_service_url: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    // Read the YAML file
    let config_yaml = fs::read_to_string("global_config.yaml")?;
    
    // Parse the YAML file
    let config: Config = serde_yaml::from_str(&config_yaml)?;

    Ok(config)
}