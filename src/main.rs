use clap::Parser;
use std::{io::Write, net::IpAddr};
use std::io;
use tokio;
use public_ip;
use nat_punching::nat_punch;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Empty"))]
    remote_nat_ip: String,

    #[arg(short('p'), long, default_value_t = 0)]
    remote_nat_port: i32,
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

#[allow(unused_macros)]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
}

#[allow(unused_macros)]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}

#[tokio::main]
async fn main() -> io::Result<()>{
    let args = Args::parse();
    let mut unused = String::new();
    
    let nat_ip = public_ip::addr().await.expect("Failed to get your external IP :(");
    println!("Welcome to NAT Punching library!");
    println!("-------------------------------------");
    println!("Your external IP: {:?}", nat_ip);
    println!("On your remote machine run: nat_punching --remote-nat-ip {:?} --remote-nat-port {:?}", nat_ip, 1212);
    println!("");
    
    let remote_nat_ip = match args.remote_nat_ip.as_str() {
        "Empty" => {
            /* Did not pass remote-nat-ip in command line so we ask here to provide it */
            print!("Enter your remote NAT IP> ");
            read!(x as IpAddr);
            x
        },
        _ => {
            /* Parse the command line argument */
            args.remote_nat_ip.parse::<IpAddr>().expect("Invalid remote NAT IP")
        }
    };

    let remote_nat_port = match args.remote_nat_port {
        0 => {
            /* Did not pass remote-nat-port in command line so we ask here to provide it */
            print!("Enter your remote NAT PORT> ");
            read!(x as i32);
            x
        }
        _ => { args.remote_nat_port }
    };
    
    let conn = nat_punch::Connection::create(nat_ip, remote_nat_ip, remote_nat_port);
    println!("{}", conn);
    println!("press <ENTER> to connect...");
    io::stdin().read_line(&mut unused).expect("Failed to readline");
    println!("Trying to punch...");
    
    conn.connect().expect("Failed to connect");

    Ok(())
}
