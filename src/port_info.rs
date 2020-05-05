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

    pub async fn build_from(address: &str, port: u16, duration: Duration) -> io::Result<PortInfo> {
        if scanner::is_port_opened(address, port, duration).await {
            println!("Discovered port {}", port);
            match scanner::get_http_banner(address, port, duration).await {             // trying to get http banner
                Ok(answer) => Ok(PortInfo::new(port, answer, ProtocolType::Http)),      // if it is possible save banner
                Err(_) => match scanner::with_timeout(                                  // if not trying to get tcp banner
                    duration,
                    scanner::get_tcp_socket_info(address, port, duration),
                )
                .await
                {
                    Ok(answer) => Ok(PortInfo::new(port, answer, ProtocolType::Tcp)),    // if it is possible save banner
                    Err(_) => match scanner::with_timeout(                               // if not scan udp banner
                        duration,
                        scanner::get_udp_socket_info(address, port, duration),
                    )
                    .await
                    {
                        Ok(answer) => Ok(PortInfo::new(port, answer, ProtocolType::Udp)),
                        Err(_) => Err(Error::new(
                            ErrorKind::NotConnected,
                            "Can't get data from socket",
                        )),
                    },
                },
            }
        } else {
            return Err(Error::new(ErrorKind::Other, "Port is closed!"));
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "|{:^6}| |{:^12}| {:^32}",
            self.port.to_string(),
            self.protocol_type.to_string(),
            self.message.trim()
        )
    }
}

#[derive(Debug)]
pub enum ProtocolType {
    Http,
    Tcp,
    Udp,
    // Https,
}

impl ProtocolType {
    fn to_string(&self) -> &str {
        match self {
            ProtocolType::Http => "Http",
            ProtocolType::Tcp => "Tcp",
            ProtocolType::Udp => "Udp",
            // ProtocolType::Https => "Https",
        }
    }
}
