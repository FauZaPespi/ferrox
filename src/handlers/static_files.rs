use crate::{http::response::Response, template_manager::render};
use std::{fs::File, io::Read, path::PathBuf};

const SERVING_DIR: &str = "www";

pub fn serve_file(file_path: &String) -> Response {
    let path = PathBuf::from(SERVING_DIR).join(file_path.trim_start_matches('/'));
    let base = PathBuf::from(SERVING_DIR).canonicalize().expect("Serving dir must exist");

    let mut canonical = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            let body = render("404", "Not Found");
            return Response {
                status: "404 Not Found".to_string(),
                body,
                content_type: "text/html".to_string(),
            };
        }
    };

    if !canonical.starts_with(&base) {
        println!("Illegal path.");
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
        Ok(_) => (),
    }

    return Response {
        status: "200 OK".to_string(),
        body: s,
        content_type: "text/html".to_string(),
    };
}
