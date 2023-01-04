use clap::Parser;
use std::{io::{self, Write}, net::IpAddr};

#[derive(Parser, Debug)]
#[command(author="Amit Berger", version, about)]
/// very simple P2P connection using UDP NAT puching
pub struct NatPunchingArgs {
    #[arg(short('i'), long)]
    remote_nat_ip: Option<IpAddr>,

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

pub fn parse_with_input_fill() -> (IpAddr, i32, i32) {
    let args = NatPunchingArgs::parse();
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
    
    (remote_nat_ip, remote_nat_port, local_port)
}