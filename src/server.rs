use std::io::{Read};
use std::net::{TcpListener, TcpStream, IpAddr};
use time::{UtcDateTime};
use std::thread;

use mime_guess::mime;

use crate::handlers::static_files::serve_file;
use crate::http::error::render_error;
use crate::http::request::Request;
use crate::http::response::Response;

pub fn serve(addr: &str) {
    let listener: TcpListener = TcpListener::bind(addr).unwrap();

    println!("Ferrox running on http://{}", addr);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        thread::spawn(|| {
            if let Err(e) = handle(stream) {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}

fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer: [u8; 1024] = [0; 1024];
    let size: usize = stream.read(&mut buffer).unwrap();
    let ip: IpAddr = stream.peer_addr()?.ip();
    let date: UtcDateTime = UtcDateTime::now();

    let request: Request = Request::parse(&buffer[..size]);
    let response: Response = match serve_file(&request.path) {
        Ok(r) => r,
        Err(_) => Response { status: "500 Internal Server Error", content_type: mime::TEXT_HTML, body: render_error("500", "Internal Server Error") }
    };

    println!("{} - [{}] \"{} {} {}\" {} {}", &ip.to_string(), &date.to_string(), &request.method, &request.path, &request.version, &response.status, &response.body.len());

    response.write_to(&mut stream)?;

    Ok(())
}