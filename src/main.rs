use async_std::net::TcpStream;
use futures::executor::block_on;
use futures::future::join_all;
use futures::join;
use std::io;


fn main() -> io::Result<()>  {
    block_on(chech_ports());
    Ok(())
}

async fn chech_ports() {
    // Join the two futures together
    let mut tasks = Vec::new();

    for port in 1..500 { 
        tasks.push(check_port(port as u16))
    }

    join!(join_all(tasks));
}


async fn check_port(port: u16){
    match TcpStream::connect(format!("13.90.224.253:{}", port)).await { 
        Ok(_) => {
            println!("PORT {} OPENED", port); 
        },
        Err(_) => println!("PORT {} CLOSED", port),
    } 
}