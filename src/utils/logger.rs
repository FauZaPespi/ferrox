use std::net::IpAddr;
use tokio::fs::OpenOptions; 
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::config::Config; 
use crate::http::{request::Request, response::Response};
use time::UtcDateTime; 

async fn append_log(config: &Config, append_file: &str, log: String) -> std::io::Result<()> {
    let file_path = format!("{}/{}", config.paths.log_dir, append_file);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&file_path)
        .await?;

    let log_line = format!("{}\n", log);
    file.write_all(log_line.as_bytes()).await?;

    Ok(())
}

pub async fn access(config: &Config, request: &Request, response: &Response, stream: &TcpStream) -> std::io::Result<()> {
    let connecting_ip: IpAddr = stream.peer_addr()?.ip();
    let requested_ip: IpAddr = stream.local_addr()?.ip();
    let date: UtcDateTime = UtcDateTime::now();

    let log: String = format!(
        "{} - [{}] \"{} {} {}\" {} {} - \"{}\" \"{}\"",
        connecting_ip.to_string(),
        date.to_string(),
        request.method,
        request.path,
        request.version,
        response.status,
        response.content_length,
        request.headers.get("User-Agent").unwrap_or(&"-".to_string()),
        requested_ip.to_string()
    );

    match append_log(config, "access.log", log).await {
        Ok(()) => { },
        Err(_) => eprintln!("Failed to persist log. Make sure directory {} exists.", config.paths.log_dir)
    };

    Ok(())
}

pub async fn error_log(config: &Config, concern: &str, error: String) {
    let date: UtcDateTime = UtcDateTime::now();
    let log: String = format!("{} [{}]: {}", date.to_string(), concern, error);

    match append_log(config, "error.log", log).await {
        Ok(()) => { },
        Err(_) => eprintln!("Failed to persist log. Make sure directory {} exists.", config.paths.log_dir)
    };
}