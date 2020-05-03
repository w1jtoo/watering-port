use std::time::Duration;

use async_std::future::timeout;
use async_std::net::TcpStream;
use async_std::prelude::*;

use hyper::http::Uri;
use hyper::Client;

use std::io::{Error, ErrorKind, Result};

use std::str;

pub async fn is_port_opened(port: u16, duration: Duration) -> bool {
    match timeout(
        duration,
        TcpStream::connect(format!("13.90.224.253:{}", port)),
    )
    .await
    {
        Ok(f) => match f {
            Ok(_) => true,
            Err(_) => false,
        },
        Err(_) => false,
    }
}
pub async fn get_http_banner(port: u16) -> Result<String> {
    let client = Client::new();
    let uri = format!("13.90.224.253:{}", port).parse::<Uri>().unwrap();
    match client.get(uri).await {
        Ok(resp) => Ok(resp.status().to_string()),
        Err(_) => Err(Error::new(ErrorKind::Other, "Can't get http answer!")),
    }
}

pub async fn get_socket_info(port: u16, duration: Duration) -> Result<String> {
    let mut buffer: Vec<u8> = vec![0; 128];
    match timeout(
        duration,
        TcpStream::connect(format!("13.90.224.253:{}", port)),
    )
    .await
    {
        Ok(connection) => match connection {
            Ok(mut stream) => match stream.write_all(b"HELLO!").await {
                Ok(_) => match stream.read(&mut buffer).await {
                    Ok(_) => Ok(str::from_utf8(&buffer).unwrap().to_string()),
                    Err(_) => Err(Error::new(
                        ErrorKind::ConnectionRefused,
                        "Can't get connection stream",
                    )),
                },
                Err(_) => Err(Error::new(
                    ErrorKind::ConnectionRefused,
                    "Can't get connection stream",
                )),
            },
            Err(_) => Err(Error::new(
                ErrorKind::ConnectionRefused,
                "Can't get connection stream",
            )),
        },
        Err(_) => Err(Error::new(ErrorKind::TimedOut, "Connection Timeout")),
    }
}
