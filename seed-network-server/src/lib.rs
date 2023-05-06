use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

pub struct NetworkServer {
    listener: TcpListener,
    configuration: NetworkConfiguration,
}

impl NetworkServer {
    pub async fn new() -> Self {
        let configuration = NetworkConfiguration {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 65535),
        };

        let listener = TcpListener::bind(configuration.addr)
            .await
            .expect("Error on bind TcpListener.");

        Self {
            listener,
            configuration,
        }
    }

    pub async fn run(&self) {
        while let Ok((stream, socket_addr)) = self.listener.accept().await {
            println!("socket_addr: {:?}", socket_addr);

            if let Ok(peer_addr) = stream.peer_addr() {
                println!("peer_addr: {:?}", peer_addr);
            }

            // let mut buffer = vec![0u8; 1024];
            // loop {
            //     let bytes_read = stream.read_buf(&mut buffer).await.unwrap();
            //     if bytes_read == 0 {
            //         break;
            //     }
            // }
            // println!("{:?}", buffer);
        }
    }
}

struct NetworkConfiguration {
    addr: SocketAddr,
}
