use std::net;
use std::fmt;
use std::io;
use std::str::FromStr;
use public_ip;

#[allow(unused_macros)]
macro_rules! validate_lock {
    ($out:ident) => {
        if $out.lock_connection {
            panic!("Cannot change connection while connected!");
        }
    };
}

pub struct UdpHoleEndpoint {
    pub local_nat_ip: net::IpAddr,
    pub remote_nat_ip: net::IpAddr,
    pub remote_nat_port: i32,
    pub local_port: i32,
    lock_connection: bool
}

impl UdpHoleEndpoint {
    pub async fn create() -> Self {
        let local_nat_ip = public_ip::addr().await.expect("Failed to get your external IP :(");
        Self{
            local_nat_ip,
            remote_nat_ip: net::IpAddr::from_str("0.0.0.0").unwrap(),
            remote_nat_port: -1,
            local_port: -1,
            lock_connection: false
        }
    }

    pub fn connect(&mut self) -> io::Result<()> {
        self.lock_connection = true;
        println!("Trying to punch...");
        /* Start udp hole punching */
        println!("Connected!");
        Ok(())
    }

    pub fn disconnect(&mut self) -> io::Result<()> {
        self.lock_connection = false;
        println!("Disconnected!");
        Ok(())
    }

    pub fn set_local_port(&mut self, port: i32) {
        validate_lock!(self);
        self.local_port = port;
    }

    pub fn set_remote_address(&mut self, remote_nat_ip: net::IpAddr, remote_nat_port: i32) {
        validate_lock!(self);
        self.remote_nat_ip = remote_nat_ip;
        self.remote_nat_port = remote_nat_port;
    }

}

impl fmt::Display for UdpHoleEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,"").unwrap();
        writeln!(f,"Connection Information").unwrap();
        writeln!(f,"--------------------------").unwrap();
        writeln!(
            f, 
            "[{:?}:{:?} (this machine)] <--> [{:?}:{:?} (remote machine)]",
            self.local_nat_ip,
            self.local_port,
            self.remote_nat_ip,
            self.remote_nat_port
        ).unwrap();
        writeln!(f,"").unwrap();
        writeln!(
            f,
            "On remote machine run: nat_punching --remote-nat-ip {} --remote-nat-port {} --local-port {}",
            self.local_nat_ip,
            self.local_port,
            self.remote_nat_port
        )
    }
}