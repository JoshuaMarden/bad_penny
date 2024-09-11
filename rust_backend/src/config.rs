// ╔═════════════════════════════ ◈{ Imports }◈ ══════════════════════════════╗

use std::collections::HashMap;
use serde::Deserialize;
use std::fs;

// ╔═══════════════════ ◈{ Functions to import Globals }◈ ════════════════════╗

/// Struct to hold browser emulation details (User-Agent, Accept headers).
#[derive(Debug, Deserialize)]
pub struct BrowserEmulationDetails {
    pub user_agent: String,
    pub accept_response: String,
}

/// Main configuration struct to hold global config data.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub ons_website: String,
    pub data_sources: HashMap<String, String>,
    pub browser_emulation_details: BrowserEmulationDetails, // This is now properly defined
}

/// Function to load the global configuration from YAML into a `Config` object.
pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_yaml = fs::read_to_string("../global_config.yml")?;
    let config: Config = serde_yaml::from_str(&config_yaml)?;
    Ok(config)
}