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
}

fn get_remote_nat_ip_from_user() -> IpAddr {
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to readline");
    input.pop();
    input.parse::<IpAddr>().expect("Invalid remote NAT IP")
}

#[tokio::main]
async fn main() -> io::Result<()>{
    let args = Args::parse();
    let mut unused = String::new();
    
    let nat_ip = public_ip::addr().await.expect("Failed to get your external IP :(");
    println!("Welcome to NAT Punching library!");
    println!("-------------------------------------");
    println!("Your external IP: {:?}", nat_ip);
    println!("On your remote machine run: nat_punching --remote-nat-ip {:?}", nat_ip);
    println!("");
    
    let remote_nat_ip = match args.remote_nat_ip.as_str() {
        "Empty" => {
            /* Did not pass remote-nat-ip in command line so we ask here to provide it */
            print!("Enter your remote NAT IP> ");
            get_remote_nat_ip_from_user()
        },
        _ => {
            /* Parse the command line argument */
            args.remote_nat_ip.parse::<IpAddr>().expect("Invalid remote NAT IP")
        }
    };
    
    let conn = nat_punch::Connection::create(nat_ip, remote_nat_ip);
    println!("{}", conn);
    println!("press <ENTER> to connect...");
    io::stdin().read_line(&mut unused).expect("Failed to readline");
    println!("Trying to punch...");
    
    conn.connect().expect("Failed to connect");

    Ok(())
}
