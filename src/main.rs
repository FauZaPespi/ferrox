mod template_manager;

use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;
use template_manager::render;

const SERVING_DIR: &str = "www";

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);
    let first_line = request.lines().next().unwrap();
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let (method, req_path, version) = (parts[0], parts[1], parts[2]);

    let path = PathBuf::from(SERVING_DIR).join(req_path.trim_start_matches('/'));

    let mut canonical = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            let body = render("404", "Not Found");
            let response = format!("HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\nServer: Ferrox\r\n\r\n{}", body.len(), body);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        }
    };

    if !canonical.starts_with(SERVING_DIR) {
        println!("Illegal path."); // TODO: Forbidden
    }

    if canonical.is_dir() {
        canonical = canonical.join("index.html");
    }

    let display = canonical.display();

    let mut file = match File::open(&canonical) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!(
            "Method: {}\nPath: {}\nVersion: {}",
            method, req_path, version
        ),
    }

    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\nServer: Ferrox\r\n\r\n{}",s.len(), s);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Ferrox running on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
