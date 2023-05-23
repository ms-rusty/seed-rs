use bevy::prelude::{Bundle, Component, Deref, DerefMut, Mut};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    io::*,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::Mutex,
    task::JoinHandle,
};

#[derive(Component)]
pub enum ConnectionEvent {
    Success((TcpStream, SocketAddr)),
    Failure(std::io::Error),
}

#[derive(Bundle)]
pub struct ConnectionBundle {
    pub connection: Connection,
    pub address: ConnectionSocketAddr,
    pub stream_reader: ConnectionStreamReader,
    pub stream_writer: ConnectionStreamWriter,
}

#[derive(Component)]
pub struct Connection;

#[derive(Component)]
pub struct ConnectionSocketAddr {
    pub socket_address: SocketAddr,
}

impl ConnectionSocketAddr {
    pub fn new(socket_address: SocketAddr) -> Self {
        Self { socket_address }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct PacketHandler(pub JoinHandle<()>);

impl PacketHandler {
    pub fn new(handle: JoinHandle<()>) -> Self {
        Self(handle)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct ConnectionStreamReader {
    pub reader: Arc<Mutex<BufReader<OwnedReadHalf>>>,
}

impl ConnectionStreamReader {
    pub fn new(read: OwnedReadHalf) -> Self {
        Self {
            reader: Arc::new(Mutex::new(BufReader::new(read))),
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct ConnectionStreamWriter {
    pub writer: Arc<Mutex<BufWriter<OwnedWriteHalf>>>,
}

impl ConnectionStreamWriter {
    pub fn new(write: OwnedWriteHalf) -> Self {
        Self {
            writer: Arc::new(Mutex::new(BufWriter::new(write))),
        }
    }
}

#[derive(Component, Debug, PartialEq, Eq)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play,
}

#[derive(Component, Debug)]
pub struct ConnectionHandshakingState;

#[derive(Component, Debug)]
pub struct ConnectionStatusState;

#[derive(Component, Debug)]
pub struct ConnectionLoginState;

#[derive(Component, Debug)]
pub struct ConnectionPlayState;

pub async fn shutdown(writer: &mut BufWriter<OwnedWriteHalf>) -> Result<()> {
    writer.shutdown().await
}
