use crate::endpoint;
use clap::Parser;
use public_ip;
use std::{io::{self, Write}, net::IpAddr};

#[derive(Parser, Debug)]
#[command(author="Amit Berger", version, about)]
/// very simple P2P connection using UDP NAT puching
pub struct NatPunchingArgs {
    #[arg(short('r'), long)]
    remote_nat_ip: Option<IpAddr>,

    #[arg(short('p'), long)]
    remote_nat_port: Option<i32>,

    #[arg(short('i'), long)]
    local_nat_ip: Option<IpAddr>,

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

pub async fn build_endpoint_from_config() -> endpoint::UdpHoleEndpoint {
    let args = NatPunchingArgs::parse();
    let local_nat_ip = match args.local_nat_ip {
        None => {
            public_ip::addr().await.expect("Failed to get your external IP :(")
        }
        Some(x) => x
    };
    let remote_nat_ip = args.remote_nat_ip.unwrap_or_else(|| -> IpAddr {
        /* Did not pass remote-nat-ip in command line so we ask here to provide it */
        print!("Enter remote NAT IP> ");
        read!(x as IpAddr);
        x
    });
    let remote_nat_port = args.remote_nat_port.unwrap_or_else(|| -> i32 {
        /* Did not pass remote-nat-port in command line so we ask here to provide it */
        print!("Enter remote NAT PORT> ");
        read!(x as i32);
        x
    });
    let local_port = args.local_port.unwrap_or_else(|| -> i32 {
        /* Did not pass local-nat-port in command line so we ask here to provide it */
        print!("Enter local PORT> ");
        read!(x as i32);
        x
    });
    
    endpoint::UdpHoleEndpoint::new(remote_nat_ip, remote_nat_port, local_nat_ip, local_port)
}