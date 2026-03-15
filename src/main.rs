mod server;
mod http;
mod handlers;
mod utils;

#[tokio::main]
async fn main() {
    server::serve("0.0.0.0:8080").await;
}