mod config;
mod endpoint;

use std::io;
use tokio;

fn wait_for_enter() {
    let mut unused = String::new();
    println!("press <ENTER> to connect...");
    io::stdin().read_line(&mut unused).expect("Failed to readline");
}

#[tokio::main]
async fn main() -> io::Result<()>{

    println!("Welcome to NAT Punching library!");
    println!("--------------------------------");
    println!();
    let mut endpoint = config::build_endpoint_from_config().await;
    println!("{endpoint}");
    wait_for_enter();
    
    endpoint.connect().expect("Failed to connect");
    endpoint.disconnect().expect("Failed to disconnect");

    Ok(())
}
