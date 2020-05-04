use futures::executor::block_on;
use futures::future::join_all;

use std::io;
use std::time::Duration;

mod port_info;

use clap::{App, Arg};

static AUTHOR: &str = "~~~~~~~~~~~~~~~~~~ |w1jtoo||wanadooht@gmail.com| ~~~~~~~~~~~~~~~~~~ ";

static ABOUT: &str = "       Tcp, udp high performed port scanner written in rust.
    Shows banners of supported application layer protocols.

    Supported protocols: 
     - http ";

#[tokio::main]
async fn main() -> io::Result<()> {
    let matches = App::new("~~~~~~~~~~~~~~~~~~~~ | watering-port |")
        .version("|0.1| ~~~~~~~~~~~~~~~~~~~~~~")
        .about(ABOUT)
        .arg(
            Arg::with_name("ADDRESS")
                .about("IPv4 of scanning target")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("port-count")
                .short('c')
                .long("port-count")
                .value_name("NUMBER")
                .about("Sets count of scanning ports")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("start-port")
                .short('s')
                .long("start-port")
                .value_name("NUMBER")
                .about("Sets first scanning port")
                .takes_value(true),
        )
        .author(AUTHOR)
        .get_matches();
    let address = matches.value_of("ADDRESS").expect("Can't get address");
    
    let port_count: usize = match matches.value_of("port-count") { 
        Some(arg_value) => arg_value.parse().expect("Wrong port count format"),
        None => 1000
    };

    let start_port: usize = match matches.value_of("count") { 
        Some(arg_value) => arg_value.parse().expect("Wrong port start_port format"),
        None => 0
    };

    print_ports_info(address, start_port, port_count);

    Ok(())
}

fn print_ports_info(address: &str, start_port: usize, port_count: usize) {
    println!("Start of scanning machine {}...", address);
    println!("Target ports: {}..{}", start_port, start_port + port_count);
    let ports = block_on(get_ports(address, Duration::from_secs(2), start_port, start_port + port_count));
    println!("|{:^6}| |{:^12}| {:^32}", "Port", "Protocol", "Banner");
    ports.iter().for_each(|p| println!("{}", p.to_string()));
}

async fn get_ports(
    address: &str,
    duration: Duration,
    start_port: usize,
    last_port: usize,
) -> Vec<port_info::PortInfo> {
    let mut tasks = Vec::new();

    for port in start_port..last_port {
        tasks.push(port_info::PortInfo::build_from(address, port as u16, duration));
    }

    let port_results = join_all(tasks).await;
    port_results
        .into_iter()
        .filter(|x| x.is_ok())
        .map(|p| p.unwrap())
        .collect()
}
