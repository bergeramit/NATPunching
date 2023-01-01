pub mod nat_punch {
    use std::net;
    use std::fmt;
    use std::io;

    pub struct Connection {
        nat_ip: net::IpAddr,
        remote_nat_ip: net::IpAddr,
        remote_nat_port: i32
    }

    impl Connection {
        pub fn create(nat_ip: net::IpAddr, remote_nat_ip: net::IpAddr, remote_nat_port: i32) -> Connection {
            Connection { nat_ip, remote_nat_ip, remote_nat_port }
        }
        pub fn connect(&self) -> io::Result<()> {
            println!("Connected!");
            Ok(())
        }
    }

    impl fmt::Display for Connection {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f,"").unwrap();
            writeln!(f,"Connection information").unwrap();
            writeln!(f,"--------------------------").unwrap();
            writeln!(f, "[{:?} (this machine)] <--> [{:?}:{:?} (remote machine)]", self.nat_ip, self.remote_nat_ip, self.remote_nat_port)
        }
    }
}

