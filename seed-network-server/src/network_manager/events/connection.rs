use std::{net::SocketAddr, sync::Arc};
use tokio::{
    io::{AsyncReadExt, BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::Mutex,
};

pub enum ConnectionEvent {
    Success(Connection),
    Failure(std::io::Error),
}

pub struct Connection {
    pub address: SocketAddr,
    pub stream_reader: Arc<Mutex<BufReader<OwnedReadHalf>>>,
    pub stream_writer: Arc<Mutex<BufWriter<OwnedWriteHalf>>>,
}

impl Connection {
    pub fn new(address: SocketAddr, mut stream: TcpStream) -> Self {
        let (a, b) = stream.into_split();

        Self {
            address,
            stream_reader: Arc::new(Mutex::new(BufReader::new(a))),
            stream_writer: Arc::new(Mutex::new(BufWriter::new(b))),
        }
    }

    pub async fn read_packet(&mut self) -> Result<(), std::io::Error> {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut value = 0;
        let mut position = 0;

        loop {
            let mut stream_reader = self.stream_reader.lock().await;
            let current_byte = stream_reader.read_u8().await?;
            value |= (current_byte & SEGMENT_BITS) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                break; // error
            }
        }

        // return value.

        Ok(())
    }

    pub async fn write_packet(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), std::io::Error> {
        self.stream.shutdown().await
    }
}
