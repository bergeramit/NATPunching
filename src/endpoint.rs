use std::net;
use std::fmt;
use std::io;
use std::{thread, time};
use std::net::{UdpSocket, Ipv4Addr};

#[allow(unused_macros)]
macro_rules! validate_disconnect {
    ($out:ident) => {
        if $out.lock_connection {
            panic!("Cannot change connection while connected!");
        }
    };
}

pub struct UdpHoleEndpoint {
    pub remote_nat_ip: net::Ipv4Addr,
    pub remote_nat_port: u16,
    pub local_nat_ip: net::Ipv4Addr,
    pub local_port: u16,
    lock_connection: bool
}

impl UdpHoleEndpoint {
    pub fn new(remote_nat_ip: net::Ipv4Addr, remote_nat_port: u16, local_nat_ip: Ipv4Addr, local_port: u16) -> Self {
        Self {
            remote_nat_ip,
            remote_nat_port,
            local_nat_ip,
            local_port,
            lock_connection: false
        }
    }

    pub fn connect(&mut self) -> io::Result<()> {
        validate_disconnect!(self);

        let mut ka_recv_buf = [12,12,12,12,12];
        let ka_buffer = [4,4,4,4];
        let sleep_timer = 100;

        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, self.local_port))?;
        socket.set_nonblocking(true).unwrap();
        self.lock_connection = true;

        println!("Trying to punch...");
        loop {
            match socket.send_to(&ka_buffer, (self.remote_nat_ip, self.remote_nat_port)) {
                Ok(_) => {},
                Err(e) => {
                    println!("Still did not go through ({:?})", e);
                }
            };
            match socket.recv_from(&mut ka_recv_buf) {
                    Ok((size, src)) => {
                        println!("Got message from: {:?}", src);
                        println!("Message size: {:?}", size);
                        println!("Message: {:?}", ka_recv_buf);
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => { /* we want to wait for the hole to be created */}
                    Err(e) => {
                        println!("Error occurred: {:?}", e);
                        break
                    },
                };
            println!("Sleeping for {sleep_timer}ms...");
            thread::sleep(time::Duration::from_millis(sleep_timer));
        };

        self.disconnect();
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.lock_connection = false;
        println!("Disconnected!");
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
            "On remote machine run: nat_punching connect --remote-nat-ip {} --remote-nat-port {} --local-port {} --local-nat-ip {}",
            self.local_nat_ip,
            self.local_port,
            self.remote_nat_port,
            self.remote_nat_ip
        )
    }
}