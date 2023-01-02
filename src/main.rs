use clap::Parser;
use std::{io::{self, Write}, net::IpAddr};
use tokio;
use nat_punching::nat_punch;

#[derive(Parser, Debug)]
#[command(author="Amit Berger", version, about)]
/// very simple P2P connection using UDP NAT puching
struct Args {
    #[arg(short('i'), long)]
    remote_nat_ip: Option<String>,

    #[arg(short('p'), long)]
    remote_nat_port: Option<i32>,

    #[arg(short('l'), long)]
    local_port: Option<i32>,
}

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        io::stdout().flush().expect("Failed to flush stdout");
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

fn get_cmd_args(args: Args) -> (IpAddr, i32, i32) {
    let remote_nat_ip = match args.remote_nat_ip {
        None => {
            /* Did not pass remote-nat-ip in command line so we ask here to provide it */
            print!("Enter remote NAT IP> ");
            read!(x as IpAddr);
            x
        },
        Some(x) => {
            /* Parse the command line argument */
            x.parse::<IpAddr>().expect("Invalid remote NAT IP")
        }
    };
    let remote_nat_port = match args.remote_nat_port {
        None => {
            /* Did not pass remote-nat-port in command line so we ask here to provide it */
            print!("Enter remote NAT PORT> ");
            read!(x as i32);
            x
        }
        Some(x) => x
    };

    let local_port = match args.local_port {
        None => {
            /* Did not pass remote-nat-port in command line so we ask here to provide it */
            print!("Enter local PORT> ");
            read!(x as i32);
            x
        }
        Some(x) => x
    };
    (remote_nat_ip, remote_nat_port, local_port)
}

#[tokio::main]
async fn main() -> io::Result<()>{
    let args = Args::parse();
    let mut unused = String::new();
    
    //let local_nat_ip = public_ip::addr().await.expect("Failed to get your external IP :(");
    let mut endpoint = nat_punch::UdpHoleEndpoint::create().await;
    println!("Welcome to NAT Punching library! (local NAT IP: {:?})", endpoint.local_nat_ip);
    println!("-------------------------------------------------------------");
    println!();
    
    let (remote_nat_ip, remote_nat_port, local_port) = get_cmd_args(args);
    endpoint.remote_nat_ip =  Some(remote_nat_ip);
    endpoint.remote_nat_port =  remote_nat_port;
    endpoint.local_port =  local_port;
    println!("{endpoint}");

    println!("On remote machine run:");
    println!(
        "nat_punching --remote-nat-ip {} --remote-nat-port {} --local-port {}",
        endpoint.local_nat_ip.unwrap(),
        endpoint.local_port,
        endpoint.remote_nat_port
    );
    println!();

    println!("press <ENTER> to connect...");
    io::stdin().read_line(&mut unused).expect("Failed to readline");
    println!("Trying to punch...");
    
    endpoint.connect().expect("Failed to connect");

    Ok(())
}
