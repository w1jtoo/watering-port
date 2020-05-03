use async_std::future::timeout;
use async_std::net::TcpStream;
use futures::executor::block_on;
use futures::future::join_all;
use std::io;
use std::time::Duration;

mod port_info;

fn main() -> io::Result<()> {
    let ports = block_on(get_ports(Duration::from_secs(2)));
    println!("ports: {:?}", ports);
    Ok(())
}

async fn get_ports(duration: Duration) -> Vec<u16> {
    let mut tasks = Vec::new();

    for port in 0..500 {
        tasks.push(check_port(port, duration));
    }

    let port_results = join_all(tasks).await;
    
    port_results
        .into_iter()
        .filter(|x| x.is_some())
        .map(|p| p.unwrap())
        .collect()
}

async fn get_info(port: u16) -> Option<port_info::PortInfo<'static>> { 
    None
}

async fn check_port(port: u16, duration: Duration) -> Option<u16> {
    println!("{}", port);

    match timeout(
        duration,
        TcpStream::connect(format!("13.90.224.253:{}", port)),
    )
    .await
    {
        Ok(f) => match f {
            Ok(_) => Some(port),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
