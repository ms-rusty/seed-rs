use std::{net::SocketAddr, sync::Arc};
use tokio::{
    io::*,
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
    pub stream: ConnectionStream,
}

impl Connection {
    pub fn new(address: SocketAddr, stream: TcpStream) -> Self {
        Self {
            address,
            stream: ConnectionStream::new(stream),
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

pub async fn read_packet(reader: &mut BufReader<OwnedReadHalf>) -> Result<i32> {
    let packet_lenght = get_varint(reader).await?;
    println!("packet_lenght: {packet_lenght}");

    let packet_id = get_varint(reader).await?;
    println!("packet_id: {:?}", packet_id);

    let protocol_version = get_varint(reader).await?;
    println!("protocol_version: {:?}", protocol_version);

    let length_server_address = get_varint(reader).await? as usize;
    println!("length_server_address: {:?}", length_server_address);
    let mut server_address = vec![0u8; length_server_address];
    reader.read_exact(&mut server_address).await?;
    println!(
        "server_address: {:?}",
        std::str::from_utf8(&server_address).unwrap()
    );

    let server_port = reader.read_u16().await?;
    println!("server_port: {:?}", server_port);

    let next_state = get_varint(reader).await?;
    println!("next_state: {:?}", next_state);

    Ok(packet_id)
}

async fn get_varint(reader: &mut BufReader<OwnedReadHalf>) -> Result<i32> {
    let mut num_read = 0;
    let mut result = 0;
    loop {
        let read = reader.read_u8().await?;
        let value = i32::from(read & 0b0111_1111);
        result |= value.overflowing_shl(7 * num_read).0;

        num_read += 1;

        if num_read > 5 {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "VarInt too long (max length: 5)",
            ));
        }
        if read & 0b1000_0000 == 0 {
            break;
        }
    }
    Ok(result)
}

pub async fn write_packet(writer: &mut BufReader<OwnedReadHalf>) -> Result<()> {
    Ok(())
}

pub async fn shutdown(writer: &mut BufWriter<OwnedWriteHalf>) -> Result<()> {
    writer.shutdown().await
}
