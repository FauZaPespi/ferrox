mod server;
mod http;
mod handlers;
mod utils;

fn main() {
    server::serve("0.0.0.0:80");
}