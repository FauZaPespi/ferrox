use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn parse(buffer: &[u8]) -> Self {
        let request: std::borrow::Cow<'_, str> = String::from_utf8_lossy(buffer);
        let mut lines = request.lines();

        let first_line: &str = lines.next().unwrap();
        let parts: Vec<&str> = first_line.split_whitespace().collect();

        let mut headers: HashMap<String, String> = HashMap::new();

        for line in lines {
            if line.is_empty() {
                break;
            }

            if let Some((key, value)) = line.split_once(':') {
                headers.insert(
                    key.trim().to_string(),
                    value.trim().to_string(),
                );
            }
        }

        Self {
            method: parts[0].to_string(),
            path: parts[1].to_string(),
            version: parts[2].to_string(),
            headers
        }
    }
}