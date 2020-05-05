use std::time::Duration;

use async_std::future::timeout;
use async_std::net::{TcpStream, UdpSocket};
use async_std::prelude::*;
use std::future::Future;

use hyper::http::Uri;
use hyper::Client;

use std::io::{Error, ErrorKind, Result};

use std::str;

pub async fn is_port_opened(address: &str, port: u16, duration: Duration) -> bool {
    match timeout(
        duration,
        TcpStream::connect(format!("{}:{}", address, port)),
    )
    .await
    {
        Ok(f) => match f {
            // it's horribly
            Ok(_) => true,
            Err(_) => false,
        },
        Err(_) => false,
    }
}
pub async fn get_http_banner(address: &str, port: u16, duration: Duration) -> Result<String> {
    let client = Client::new();
    let uri = format!("http://{}:{}", address, port)
        .parse::<Uri>()
        .unwrap();
    match timeout(duration, client.get(uri)).await {
        Ok(resp) => match resp {
            Ok(resp) => Ok(resp.status().to_string()),
            Err(_) => Err(Error::new(ErrorKind::Other, "Can't get http answer!")),
        },
        Err(_) => Err(Error::new(ErrorKind::TimedOut, "Connection Timeout")),
    }
}

pub async fn with_timeout<F>(duration: Duration, f: F) -> Result<String>
where
    F: Future<Output = Result<String>>,
{
    match timeout(duration, f).await {
        Ok(answ) => match answ {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        },
        Err(_) => Ok("port is blocked by a firewall".to_string()),
    }
}

pub async fn get_tcp_socket_info(address: &str, port: u16, duration: Duration) -> Result<String> {
    let mut buffer: Vec<u8> = vec![0; 128];
    match timeout(
        duration,
        TcpStream::connect(format!("{}:{}", address, port)),
    )
    .await
    {
        Ok(connection) => match connection {
            Ok(mut stream) => match stream.write_all(b"HELLO!").await {
                Ok(_) => match stream.read(&mut buffer).await {
                    Ok(_) => Ok(str::from_utf8(&buffer)
                        .unwrap_or("can't convert banner info to UTF-8")
                        .to_string()),
                    Err(_) => Err(Error::new(
                        ErrorKind::ConnectionRefused,
                        "Can't get connection stream",
                    )),
                },
                Err(_) => Ok("port is blocked by a firewall".to_string()),
            },
            Err(_) => Err(Error::new(
                ErrorKind::ConnectionRefused,
                "Can't get connection stream",
            )),
        },
        Err(_) => Err(Error::new(ErrorKind::TimedOut, "Connection Timeout")),
    }
}

pub async fn get_udp_socket_info(address: &str, port: u16, duration: Duration) -> Result<String> {
    let mut buffer: Vec<u8> = vec![0; 128];
    let address_to = format!("{}:{}", address, port);
    match timeout(duration, UdpSocket::bind("127.0.0.1:34254")).await {
        Ok(opt_socket) => match opt_socket {
            Ok(socket) => match socket.connect(address_to).await {
                Ok(_) => match socket.send(b"HELLO!").await {
                    Ok(_) => match socket.recv_from(&mut buffer).await {
                        Ok(_) => Ok(str::from_utf8(&buffer).unwrap().to_string()),
                        Err(_) => Ok("port is blocked by a firewall".to_string()),
                    },
                    Err(_) => Ok("port is blocked by a firewall".to_string()),
                },
                Err(_) => Ok("port is blocked by a firewall".to_string()),
            },
            Err(_) => Err(Error::new(ErrorKind::TimedOut, "Connection Timeout")),
        },
        Err(_) => Err(Error::new(ErrorKind::TimedOut, "Connection Timeout")),
    }
}
