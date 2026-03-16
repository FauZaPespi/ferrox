use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub addr: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PathsConfig {
    pub serve_dir: String,
    pub log_dir: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub paths: PathsConfig,
    #[serde(default)] // Если секции нет в файле, создаст пустой HashMap (чтобы сервер не падал)
    pub headers: HashMap<String, String>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str: String = fs::read_to_string(path)?;
        
        let config: Config = serde_yaml::from_str(&config_str)?; 
        
        Ok(config)
    }
}