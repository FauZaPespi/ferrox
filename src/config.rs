use serde::Deserialize;
use std::{collections::HashMap, fs};

/// Stores the network address settings used when binding the HTTP server.
#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub addr: String,
}

/// Defines the filesystem paths used for static files and log output.
#[derive(Deserialize, Debug, Clone)]
pub struct PathsConfig {
    pub serve_dir: String,
    pub log_dir: String
}

/// Represents the full Ferrox configuration loaded from `ferrox-compose.yml`.
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub paths: PathsConfig,
    #[serde(default)] // Если секции нет в файле, создаст пустой HashMap (чтобы сервер не падал)
    pub headers: HashMap<String, String>,
}

impl Config {
    /// Loads and deserializes the YAML configuration file from disk.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the YAML configuration file.
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str: String = fs::read_to_string(path)?;
        
        let config: Config = serde_yaml::from_str(&config_str)?; 
        
        Ok(config)
    }
}
