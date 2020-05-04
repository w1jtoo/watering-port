use futures::executor::block_on;
use futures::future::join_all;

use std::io;
use std::time::Duration;

mod port_info;

use clap::App;


#[tokio::main]
async fn main() -> io::Result<()> {
    let matches = App::new("watering-port")
        .version("1.0")
        .author("w1jtoo wanadooht@gmail.com")
        .about("Smth about")
        .arg("<ADDRESS>              'IP-adress of sec'")
        .get_matches();
    print_ports_info(matches.value_of("ADDRESS").expect("Can't get address"));

    Ok(())
}

fn print_ports_info(address: &str) {
    let ports = block_on(get_ports(address, Duration::from_secs(2)));
    ports.iter().for_each(|p| println!("{}", p.to_string()));
}

async fn get_ports(address: &str, duration: Duration) -> Vec<port_info::PortInfo> {
    let mut tasks = Vec::new();

    for port in 0..500 {
        tasks.push(port_info::PortInfo::build_from(address, port, duration));
    }

    let port_results = join_all(tasks).await;
    port_results
        .into_iter()
        .filter(|x| x.is_ok())
        .map(|p| p.unwrap())
        .collect()
}
