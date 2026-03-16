mod server;
mod http;
mod handlers;
mod utils;
mod config;
use config::Config;
use std::sync::Arc;

/// Loads the application configuration file and starts the Ferrox HTTP server.
#[tokio::main]
async fn main() {
    let config: Config = config::Config::load("ferrox-compose.yml").expect("Failed to load ferrox-compose.yml");
    let shared_config: Arc<Config> = Arc::new(config);
    server::serve(shared_config).await;
}