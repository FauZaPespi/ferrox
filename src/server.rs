use std::io::{Error, ErrorKind, Result};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::sync::Arc;

use crate::handlers::static_files::serve_file;
use crate::http::request::Request;
use crate::http::response::{Body, Response};
use crate::utils::logger;
use crate::config::Config;

const MAX_HEADER_SIZE: u64 = 8192; // 8KB
const CONNECTION_TIMEOUT_SEC: u64 = 10;

pub async fn serve(config: Arc<Config>) {
    let addr = format!("{}:{}", config.server.addr, config.server.port);
    let listener = TcpListener::bind(&addr).await.expect(&format!("Ferrox failed to bind on http://{addr}"));

    println!("Ferrox running on http://{addr}");

    loop {
        let (stream, _) = match listener.accept().await {
            Ok(res) => res,
            Err(e) => {
                logger::error_log(&config, "core", format!("Failed to accept: {}", e)).await;
                continue;
            }
        };

        let task_config: Arc<Config> = Arc::clone(&config);
        let log_config: Arc<Config> = Arc::clone(&config);

        tokio::spawn(async move {
            let duration = Duration::from_secs(CONNECTION_TIMEOUT_SEC);

            match tokio::time::timeout(duration, handle(stream, task_config)).await {
                Ok(Err(e)) => {
                    logger::error_log(&log_config, "core", format!("Connection error: {}", e)).await;
                }
                Err(_) => {
                    logger::error_log(&log_config, "core", "Connection timed out".to_string()).await;
                }
                Ok(Ok(())) => {
                }
            }
        });
    }
}

async fn handle(mut stream: TcpStream, config: Arc<Config>) -> Result<()> {
    let mut full_data: Vec<u8> = Vec::new();
    let mut temp_buffer: [u8; 1024] = [0u8; 1024];

    loop {
        let bytes_read = stream.read(&mut temp_buffer).await?;

        if bytes_read == 0 {
            return Ok(());
        }

        full_data.extend_from_slice(&temp_buffer[..bytes_read]);

        if full_data.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }

        if full_data.len() > MAX_HEADER_SIZE as usize {
            return Err(Error::new(
                ErrorKind::ArgumentListTooLong,
                "Max header size reached.",
            ));
        }
    }

    let request = match Request::parse(&full_data) {
        Ok(r) => r,
        Err(e) => {
            logger::error_log(&config, "parser", format!("Failed to parse http request: {}", e)).await;

            let error_res = Response::error("400", "Bad Request");
            let _ = error_res.write_headers(&mut stream, &config).await?;
            if let Body::Bytes(b) = error_res.body {
                let _ = stream.write_all(&b).await;
            }

            return Ok(());
        }
    };

    let mut response: Response = match serve_file(&request.path, config.paths.serve_dir.clone()).await {
        Ok(r) => r,
        Err(e) => {
            logger::error_log(&config,"file", format!("Failed to server static file: {}", e)).await;
            Response::error("500", "Internal Server Error")
        }
    };

    response.write_headers(&mut stream, &config).await?;

    match &mut response.body {
        Body::Bytes(bytes) => {
            stream.write_all(bytes).await?;
        }
        Body::File(file) => {
            tokio::io::copy(file, &mut stream).await?;
        }
    }

    logger::access(&config, &request, &response, &stream).await?;

    Ok(())
}
