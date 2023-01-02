use clap::Parser;
use std::{io::Write, net::IpAddr};
use std::io;
use tokio;
use public_ip;
use nat_punching::nat_punch::{self};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Empty"))]
    remote_nat_ip: String,

    #[arg(short('p'), long, default_value_t = 0)]
    remote_nat_port: i32,

    #[arg(short('l'), long, default_value_t = 0)]
    local_port: i32,
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

    let local_port = match args.local_port {
        0 => {
            /* Did not pass remote-nat-port in command line so we ask here to provide it */
            print!("Enter your local PORT> ");
            read!(x as i32);
            x
        }
        _ => { args.local_port }
    };
    (remote_nat_ip, remote_nat_port, local_port)
}

#[tokio::main]
async fn main() -> io::Result<()>{
    let args = Args::parse();
    let mut unused = String::new();
    
    let local_nat_ip = public_ip::addr().await.expect("Failed to get your external IP :(");
    println!("Welcome to NAT Punching library!");
    println!("-------------------------------------");
    
    let (remote_nat_ip, remote_nat_port, local_port) = get_cmd_args(args);
    println!("Your local NAT IP: {:?}", local_nat_ip);
    println!("On your remote machine run: nat_punching --remote-nat-ip {local_nat_ip} --remote-nat-port {local_port} --local-port {remote_nat_port}");
    println!("");
    
    let conn = nat_punch::Connection{local_nat_ip, remote_nat_ip, remote_nat_port, local_port};
    println!("{conn}");
    println!("press <ENTER> to connect...");
    io::stdin().read_line(&mut unused).expect("Failed to readline");
    println!("Trying to punch...");
    
    conn.connect().expect("Failed to connect");

    Ok(())
}
