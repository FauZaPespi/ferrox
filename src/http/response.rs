use std::io::Write;
use mime_guess::Mime;

pub struct Response {
    pub status: &'static str,
    pub content_type: Mime,
    pub body: Vec<u8>,
}

impl Response {
    pub fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let headers = format!(
            "HTTP/1.1 {}\r\n\
             Content-Type: {}\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             Server: Ferrox\r\n\
             \r\n",
            self.status,
            self.content_type.to_string(),
            self.body.len()
        );

        writer.write_all(headers.as_bytes())?;
        writer.write_all(&self.body)?;

        Ok(())
    }
}