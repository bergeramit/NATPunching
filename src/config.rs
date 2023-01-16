use crate::endpoint;
use clap::Parser;
use public_ip;
use std::{io::{self, Write}, net::Ipv4Addr};

#[derive(Parser)]
#[command(author="Amit Berger", version, about)]
/// very simple P2P connection using UDP NAT puching

pub struct Args {
    #[command(subcommand)]
    pub action: Action
}

#[derive(clap::Subcommand)]
pub enum Action {
    Connect {
        #[arg(short('r'), long, help="Your peer's NAT IP ( this is your peer's --local-nat-ip)")]
        remote_nat_ip: Option<Ipv4Addr>,

        #[arg(short('p'), long, help="The port on the NAT IP you peer will listen to for the connection ( this is your peer's --local-port)")]
        remote_nat_port: Option<u16>,

        #[arg(short('i'), long, help="Your NAT's IP ( this is your peer's --remote-nat-ip)")]
        local_nat_ip: Option<Ipv4Addr>,

        #[arg(short('l'), long, help="The local port you will be listening for ( this is your peer's --remote-nat-port)")]
        local_port: Option<u16>,
    },
    DisplayNatIP
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

pub async fn build_endpoint_from_connect_command(
    local_nat_ip: Option<Ipv4Addr>,
    local_port: Option<u16>,
    remote_nat_ip: Option<Ipv4Addr>,
    remote_nat_port: Option<u16>) -> endpoint::UdpHoleEndpoint {
    let local_nat_ip = match local_nat_ip {
        None => {
            public_ip::addr_v4().await.expect("Failed to get your external IP :(")
        }
        Some(x) => x
    };
    let remote_nat_ip = remote_nat_ip.unwrap_or_else(|| -> Ipv4Addr {
        /* Did not pass remote-nat-ip in command line so we ask here to provide it */
        print!("Enter remote NAT IP> ");
        read!(x as Ipv4Addr);
        x
    });
    let remote_nat_port = remote_nat_port.unwrap_or_else(|| -> u16 {
        /* Did not pass remote-nat-port in command line so we ask here to provide it */
        print!("Enter remote NAT PORT> ");
        read!(x as u16);
        x
    });
    let local_port = local_port.unwrap_or_else(|| -> u16 {
        /* Did not pass local-nat-port in command line so we ask here to provide it */
        print!("Enter local PORT> ");
        read!(x as u16);
        x
    });
    
    endpoint::UdpHoleEndpoint::new(remote_nat_ip, remote_nat_port, local_nat_ip, local_port)
}