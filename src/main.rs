mod config;
mod endpoint;

use std::io;
use tokio;
use clap::Parser;
use config::{Action, Args};

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
    let args = Args::parse();
    let mut endpoint = match args.action {
        Action::DisplayNatIP => {
            let local_nat_ip = public_ip::addr_v4().await.expect("Failed to get your external IP :(");
            println!("local nat ip: {:?}", local_nat_ip);
            return Ok(())
        },
        Action::Connect {
            remote_nat_ip,
            remote_nat_port,
            local_nat_ip,
            local_port } => {
                config::build_endpoint_from_connect_command(
                    remote_nat_ip,
                    remote_nat_port,
                    local_nat_ip,
                    local_port).await
        }
    };
    
    println!("{endpoint}");
    wait_for_enter();
    
    endpoint.connect().expect("Failed to connect");
    endpoint.disconnect();

    Ok(())
}
