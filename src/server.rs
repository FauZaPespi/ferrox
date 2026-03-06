use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::http::request::{Request};
use crate::handlers::static_files::{serve_file};
use crate::http::response::Response;

pub fn serve(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();

    println!("Ferrox running on http://{}", addr);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        thread::spawn(|| {
            handle(stream);
        });
    }
}

fn handle(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request: Request = Request::parse(&buffer);
    let response: Response = serve_file(&request.path);

    println!("{} {} {}", &request.method, &request.path, &request.version);

    stream.write(response.build().as_bytes()).unwrap();
    stream.flush().unwrap();
}