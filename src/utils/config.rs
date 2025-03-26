use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct DebugConfig {
    pub tty_path: String,
}

#[derive(Debug, Deserialize)]
pub struct MapConfig {
    pub width: usize,
    pub height: usize,
    pub seed: u32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub debug: DebugConfig,
    pub map: MapConfig,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
