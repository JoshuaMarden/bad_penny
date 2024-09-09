// ╔═════════════════════════════ ◈{ Imports }◈ ══════════════════════════════╗


use tokio;
use reqwest;
use sqlx::postgres::PgPoolOptions;
use std::env;
use serde::Deserialize;
use serde_yaml;
use std::fs;

// ╔═════════════════════════ ◈{ Import  Globals }◈ ══════════════════════════╗


#[derive(Debug, Deserialize)]
pub struct Config {
    pub data_sources: DataSources,
}

#[derive(Debug, Deserialize)]
pub struct DataSources {
    pub ons_cpi_api: String,
    pub ons_nominal_petrol_api: String,
    pub ons_nominal_diesel_api: String,
    pub ons_cpih_gas_api: String,
    pub ons_cpih_electricity_api: String,
}

// Public function to load and print the config
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    println!("{:?}", config);

    Ok(())
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    // Load globals from yaml by specifying the relative path from rust_backend to the project root
    let config_yaml = fs::read_to_string("../global_config.yml")?;
    let config: Config = serde_yaml::from_str(&config_yaml)?;
    Ok(config)
}