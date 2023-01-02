pub mod nat_punch {
    use std::net;
    use std::fmt;
    use std::io;
    use public_ip;

    #[derive(Default)]
    pub struct UdpHoleEndpoint {
        pub local_nat_ip: Option<net::IpAddr>,
        pub remote_nat_ip: Option<net::IpAddr>,
        pub remote_nat_port: i32,
        pub local_port: i32,
    }

    impl UdpHoleEndpoint {
        pub async fn create() -> Self {
            let local_nat_ip = public_ip::addr().await.expect("Failed to get your external IP :(");
            Self{local_nat_ip: Some(local_nat_ip), ..Default::default()}
        }

        pub fn connect(&self) -> io::Result<()> {
            println!("Connected!");
            Ok(())
        }
    }

    impl fmt::Display for UdpHoleEndpoint {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f,"").unwrap();
            writeln!(f,"UdpHoleEndpoint information").unwrap();
            writeln!(f,"--------------------------").unwrap();
            writeln!(
                f, 
                "[{:?}:{:?} (this machine)] <--> [{:?}:{:?} (remote machine)]",
                self.local_nat_ip.unwrap(),
                self.local_port,
                self.remote_nat_ip.unwrap(),
                self.remote_nat_port
            )
        }
    }
}

