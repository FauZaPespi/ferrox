pub struct Response {
    pub status: String,
    pub body: String,
    pub content_type: String,
}

impl Response {
    pub fn build(&self) -> String {
        format!(
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nServer: Ferrox\r\n\r\n{}",
            self.status,
            self.content_type,
            self.body.len(),
            self.body
        )
    }
}