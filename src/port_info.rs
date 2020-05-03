use std::io;
use std::io::{Error, ErrorKind};
use std::time::Duration;

mod scanner;

#[derive(Debug)]
pub struct PortInfo {
    pub protocol_type: ProtocolType,
    pub message: String,
    pub port: u16,
}

impl PortInfo {
    const fn new(_port: u16, msg: String, ptype: ProtocolType) -> PortInfo {
        PortInfo {
            port: _port,
            message: msg,
            protocol_type: ptype,
        }
    }

    pub async fn build_from(port: u16, duration: Duration) -> io::Result<PortInfo> {
        if scanner::is_port_opened(port, duration).await {
            match scanner::get_http_banner(port).await {
                Ok(answer) => Ok(PortInfo::new(port, answer, ProtocolType::Http)),
                Err(_) => match scanner::get_socket_info(port, Duration::from_secs(2)).await {
                    Ok(answer) => Ok(PortInfo::new(port, answer, ProtocolType::Http)),
                    Err(_) => Err(Error::new(ErrorKind::NotConnected, "Can't get data from socket")),
                },
            }
        } else {
            return Err(Error::new(ErrorKind::Other, "Port is closed!")); // TODO: refactor scanner
        }
    }

    pub fn to_string(&self) -> String {
        // "".to_string()
        format!(
            "|{}| |{}| {}",
            self.port.to_string(),
            self.protocol_type.to_string(),
            self.message
        )
    }
}

#[derive(Debug)]
pub enum ProtocolType {
    Http,
    Tcp,
    Udp,
    Https,
}

impl ProtocolType {
    pub fn to_string(&self) -> &str {
        match self {
            Http => "Http",
            Tcp => "Tcp",
            Udp => "Udp",
            Https => "Https",
        }
    }
}
