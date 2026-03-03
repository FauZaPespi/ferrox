use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer));

    let response: &str = "HTTP/1.1 200 OK\r\n\r\nHello, Ferrox!";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:80").unwrap();

    println!("Server running on http://127.0.0.1:80");

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_connection(stream);
    }
}