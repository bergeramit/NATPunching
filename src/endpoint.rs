use std::net;
use std::fmt;
use std::io;
use std::net::{IpAddr, ToSocketAddrs, UdpSocket};

#[allow(unused_macros)]
macro_rules! validate_lock {
    ($out:ident) => {
        if $out.lock_connection {
            panic!("Cannot change connection while connected!");
        }
    };
}

pub struct UdpHoleEndpoint {
    pub remote_nat_ip: net::IpAddr,
    pub remote_nat_port: u16,
    pub local_nat_ip: net::IpAddr,
    pub local_port: u16,
    lock_connection: bool
}

impl UdpHoleEndpoint {
    pub fn new(remote_nat_ip: net::IpAddr, remote_nat_port: u16, local_nat_ip: IpAddr, local_port: u16) -> Self {
        Self{
            remote_nat_ip,
            remote_nat_port,
            local_nat_ip,
            local_port,
            lock_connection: false
        }
    }

    pub fn connect(&mut self) -> io::Result<()> {
        validate_lock!(self);
        let mut recv_buf = [0; 10];
        let mut send_buf = [0; 10];

        let send_socket = UdpSocket::bind((self.remote_nat_ip, self.remote_nat_port))?;
        let recv_socket = UdpSocket::bind((self.local_nat_ip, self.local_port))?;
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