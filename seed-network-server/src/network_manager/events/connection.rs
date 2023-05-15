use std::net::SocketAddr;
use tokio::{io::BufWriter, net::TcpStream};

pub enum ConnectionEvent {
    Success(Connection),
    Failure(std::io::Error),
}

pub struct Connection {
    pub address: SocketAddr,
    pub stream: BufWriter<TcpStream>,
}

impl Connection {
    pub fn new(address: SocketAddr, stream: TcpStream) -> Self {
        Self {
            address,
            stream: BufWriter::new(stream),
        }
    }

    pub async fn read_packet(&mut self) -> Result<()> {}

    pub async fn write_packet(&mut self) -> Result<()> {}
}
