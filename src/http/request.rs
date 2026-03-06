pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
}

impl Request {
    pub fn parse(buffer: &[u8]) -> Self {
        let request: std::borrow::Cow<'_, str> = String::from_utf8_lossy(buffer);
        let first_line: &str = request.lines().next().unwrap();
        let parts: Vec<&str> = first_line.split_whitespace().collect();

        Self {
            method: parts[0].to_string(),
            path: parts[1].to_string(),
            version: parts[2].to_string(),
        }
    }
}