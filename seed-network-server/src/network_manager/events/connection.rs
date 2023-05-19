use bytes::BytesMut;
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    io::*,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::Mutex,
};

use crate::network::Packet;

pub enum ConnectionEvent {
    Success(Connection),
    Failure(std::io::Error),
}

pub struct Connection {
    pub address: SocketAddr,
    pub stream: ConnectionStream,
    pub state: ConnectionState,
}

impl Connection {
    pub fn new(address: SocketAddr, stream: TcpStream) -> Self {
        Self {
            address,
            stream: ConnectionStream::new(stream),
            state: ConnectionState::Handshaking,
        }
    }
}

pub struct ConnectionStream {
    pub reader: Arc<Mutex<BufReader<OwnedReadHalf>>>,
    pub writer: Arc<Mutex<BufWriter<OwnedWriteHalf>>>,
}

impl ConnectionStream {
    pub fn new(stream: TcpStream) -> Self {
        let (read, write) = stream.into_split();
        let bufreader = BufReader::new(read);
        let bufwriter = BufWriter::new(write);

        Self {
            reader: Arc::new(Mutex::new(bufreader)),
            writer: Arc::new(Mutex::new(bufwriter)),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play,
}

pub async fn read_packet(
    reader: &mut BufReader<OwnedReadHalf>,
) -> anyhow::Result<Packet, anyhow::Error> {
    let mut buffer = BytesMut::with_capacity(4 * 1024);
    reader.read_buf(&mut buffer).await?;

    if buffer.is_empty() {
        return Err(anyhow::anyhow!("buffer empty!"));
    }

    Ok(Packet::new(buffer))
}

pub async fn write_packet(writer: &mut BufReader<OwnedReadHalf>) -> Result<()> {
    Ok(())
}

pub async fn shutdown(writer: &mut BufWriter<OwnedWriteHalf>) -> Result<()> {
    writer.shutdown().await
}
