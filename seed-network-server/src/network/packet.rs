use bytes::{Bytes, BytesMut};

use super::packet_reader::PacketReader;

#[derive(Debug, thiserror::Error)]
pub enum PacketError {
    #[error("Unexpected end of packet. {0}")]
    UnexpectedEndOfPacket(String),

    #[error("Invalid packet. {0}")]
    InvalidPacket(String),

    #[error("Invalid VarInt packet.")]
    InvalidVarIntPacket,
}

// https://wiki.vg/Protocol#Packet_format
pub struct Packet {
    pub length: i32,
    pub command: i32,
    pub data: Bytes,
}

impl Packet {
    pub fn new(mut data: BytesMut) -> Result<Self, anyhow::Error> {
        let mut reader = PacketReader::new(&data);
        let packet_length_var_int = reader.read_var_int()?;
        let packet_command_var_int = reader.read_var_int()?;

        let data: Bytes = data
            .split_off(packet_length_var_int.position + packet_command_var_int.position)
            .freeze();

        Ok(Self {
            length: packet_length_var_int.value,
            command: packet_command_var_int.value,
            data,
        })
    }
}
