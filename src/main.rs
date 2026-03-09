mod server;
mod http;
mod handlers;

fn main() {
    server::serve("0.0.0.0:80");
}