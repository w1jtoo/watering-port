use std::io;
use std::time::Duration;

#[derive(Debug)]
pub struct PortInfo<'a> {
    pub protocol_type: ProtocolType,
    pub message: &'a str,
    pub port: u8,
}

impl<'a> PortInfo<'a> {
    const fn new(_port: u8) -> PortInfo<'a> {
        PortInfo {
            port: _port,
            message: "unrecognized",
            protocol_type: ProtocolType::Tcp,
        }
    }

    pub async fn build_from(port: u8) -> io::Result<PortInfo<'a>> { 
        Ok(PortInfo::new(port))
    } 

    pub fn to_string(&self) -> String { 
        // "".to_string()
        format!("|{}| |{}| {}", self.port.to_string(), self.protocol_type.to_string(), self.message)
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