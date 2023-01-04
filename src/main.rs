mod args;
mod endpoint;

use std::io;
use endpoint::UdpHoleEndpoint;
use tokio;

fn wait_for_enter() {
    let mut unused = String::new();
    println!("press <ENTER> to connect...");
    io::stdin().read_line(&mut unused).expect("Failed to readline");
}

#[tokio::main]
async fn main() -> io::Result<()>{
    
    let mut endpoint = UdpHoleEndpoint::create().await;
    println!("Welcome to NAT Punching library! (local NAT IP: {:?})", endpoint.local_nat_ip);
    println!("-------------------------------------------------------------");
    println!();

    let (remote_nat_ip, remote_nat_port, local_port) = args::parse_with_input_fill();
    endpoint.set_local_port(local_port);
    endpoint.set_remote_address(remote_nat_ip, remote_nat_port);
    

    println!("{endpoint}");
    wait_for_enter();
    
    endpoint.connect().expect("Failed to connect");
    endpoint.disconnect().expect("Failed to disconnect");

    Ok(())
}
