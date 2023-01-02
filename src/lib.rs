pub mod nat_punch {
    use std::net;
    use std::fmt;
    use std::io;

    pub struct Connection {
        pub local_nat_ip: net::IpAddr,
        pub remote_nat_ip: net::IpAddr,
        pub remote_nat_port: i32,
        pub local_port: i32,
    }

    impl Connection {
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
            writeln!(
                f, 
                "[{:?}:{:?} (this machine)] <--> [{:?}:{:?} (remote machine)]",
                self.local_nat_ip,
                self.local_port,
                self.remote_nat_ip,
                self.remote_nat_port
            )
        }
    }
}

