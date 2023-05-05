use tokio::{io::AsyncReadExt, net::TcpListener};

pub struct NetworkServer {
    listener: TcpListener,
}

impl NetworkServer {
    pub fn new(listener: TcpListener) -> Self {
        Self { listener }
    }

    pub async fn run(&self) {
        while let Ok((mut stream, socket_addr)) = self.listener.accept().await {
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
