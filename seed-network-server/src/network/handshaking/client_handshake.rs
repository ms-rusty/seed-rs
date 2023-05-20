use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::network::{
    packet::PacketError, packet_reader::PacketReader, ClientHandshakingPackets, Packet, VarInt,
};

#[derive(Debug)]
pub struct PacketClientHandshake<'packet> {
    pub protocol_version: VarInt,
    pub server_address: &'packet str,
    pub server_port: u16,
    pub next_state: HandshakeState,
}

impl<'a> TryFrom<&'a Packet> for PacketClientHandshake<'a> {
    type Error = anyhow::Error;

    fn try_from(packet: &'a Packet) -> Result<Self, Self::Error> {
        if packet.command != ClientHandshakingPackets::Handshake as i32 {
            return Err(PacketError::InvalidPacket(format!(
                "Received Command: {}, Client Packet: ClientHandshakingPackets::Handshake",
                packet.command
            ))
            .into());
        }

        let mut reader = PacketReader::from(packet);
        let protocol_version = reader.read_var_int()?;
        let server_address = reader.read_str()?;
        let server_port = reader.read_u16()?;
        let next_state = HandshakeState::new(reader.read_var_int()?)
            .ok_or_else(|| PacketError::InvalidPacket("Unknown handshakeState.".to_owned()))?;

        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}

#[derive(FromPrimitive, Debug, PartialEq, Eq, Clone)]
pub enum HandshakeState {
    Status = 1,
    Login = 2,
}

impl HandshakeState {
    pub fn new(var_int: VarInt) -> Option<Self> {
        FromPrimitive::from_i32(var_int.value)
    }
}
