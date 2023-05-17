use std::{error::Error, io::ErrorKind, net::SocketAddr};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

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

    pub async fn read_packet(&mut self) -> Result<(), std::io::Error> {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut value = 0;
        let mut position = 0;

        loop {
            let current_byte = self.stream.read_u8().await?;
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
