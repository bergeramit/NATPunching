use clap::Parser;
use std::{io, net::IpAddr};
use public_ip;
use tokio;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Empty"))]
    target_nat_ip: String,
}

async fn print_external_ip() -> Option<IpAddr> {
    // Attempt to get an IP address and print it.
    public_ip::addr().await
}

#[tokio::main]
async fn main() -> io::Result<()>{
    let args = Args::parse();
    let mut unused = String::new();

    let external_ip = print_external_ip().await;
    let external_ip = match external_ip {
        Some(x) => x,
        None => panic!("Failed to get your external IP :(")
    };
    
    println!("Welcome to NAT Punching library!");
    println!("Both you and your peer should get each other's external IPs");
    println!("Your external IP: {:?} (tell your peer :))", external_ip);
    let target_nat_ip = match args.target_nat_ip.as_str() {
        "Empty" => {
            println!("Please enter your peer's external IP:");
            let mut target_nat_ip_input = String::new();
            io::stdin().read_line(&mut target_nat_ip_input).expect("failed to readline...");
            //let target_nat_ip_input = tokio::io::read_to_string(io::stdin())?;
            println!("entered: {:?}", target_nat_ip_input);
            target_nat_ip_input.as_str().parse::<IpAddr>().expect("Invalid target NAT IP")
        },
        _ => args.target_nat_ip.parse::<IpAddr>().expect("Invalid target NAT IP")
    };
    
    println!("");
    println!("Connection information:");
    println!("[Your NAT IP ({:?})] <--> [ Target NAT IP ({:?})]", external_ip, target_nat_ip);

    println!("");
    println!("On your target machine run: nat_punching --target-nat-ip {:?}", external_ip);
    println!("press <ENTER> to continue to establish connection...");
    io::stdin().read_line(&mut unused).expect("failed to readline...");
    println!("Trying to punch {:?}...", target_nat_ip);

    Ok(())
}
