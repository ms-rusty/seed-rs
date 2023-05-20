use seed_network_common::VarInt;

use crate::network_manager::packets::{
    client::packets::ClientHandshakingPackets,
    packet_errors::{PacketError, PacketReaderError},
    packet_reader::PacketReader,
    Packet,
};

#[derive(Debug)]
pub struct ClientHandshakePacket<'packet> {
    pub protocol_version: VarInt,
    pub server_address: &'packet str,
    pub server_port: u16,
    pub next_state: NextState,
}

impl<'packet> TryFrom<&'packet Packet> for ClientHandshakePacket<'packet> {
    type Error = PacketError;

    fn try_from(packet: &'packet Packet) -> Result<Self, Self::Error> {
        if packet.id != ClientHandshakingPackets::Handshake {
            return Err(Self::Error::InvalidPacket);
        }

        let mut reader = PacketReader::from(packet);
        let protocol_version = reader.read_var_int()?;
        let server_address = reader.read_str()?;
        let server_port = reader.read_u16()?;
        let next_state = reader.read_var_int()?;
        let next_state = NextState::try_from(next_state)?;

        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}

#[derive(Debug)]
pub enum NextState {
    Status,
    Login,
}

impl TryFrom<VarInt> for NextState {
    type Error = PacketReaderError;

    fn try_from(var_int: VarInt) -> Result<Self, Self::Error> {
        match var_int.value {
            1 => Ok(Self::Status),
            2 => Ok(Self::Login),
            _ => Err(Self::Error::InvalidNextState),
        }
    }
}
