mod server;
mod http;
mod handlers;
mod template_manager;

fn main() {
    server::serve("127.0.0.1:80");
}