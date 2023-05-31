use bevy::prelude::Component;
use bytes::BytesMut;
use seed_network_server_common::VarInt;

use crate::network_packet_manager::components::packets::{
    packet_errors::{PacketError, PacketReaderError},
    packet_reader::PacketReader,
    Packet,
};

use super::ClientHandshakingPackets;

#[derive(Component, Debug)]
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

        // Criamos um novo pacote no next state.
        let packet = if reader.remaining() > 0 {
            let mut data = BytesMut::from(reader.get_remaining_bytes()?);

            let mut reader = PacketReader::new(&data);
            let packet_length = reader.read_var_int()?;
            let packet_id = reader.read_var_int()?;

            let buffer = data
                .split_off(packet_length.position + packet_id.position)
                .freeze();

            Some(Packet {
                id: packet_id,
                data: buffer,
            })
        } else {
            None
        };

        let next_state = NextState::try_from((next_state, packet))?;

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
    Status(Option<Packet>),
    Login(Packet),
}

impl TryFrom<(VarInt, Option<Packet>)> for NextState {
    type Error = PacketReaderError;

    fn try_from((var_int, packet): (VarInt, Option<Packet>)) -> Result<Self, Self::Error> {
        match var_int.value {
            1 => Ok(Self::Status(packet)),
            2 => {
                if let Some(packet) = packet {
                    Ok(Self::Login(packet))
                } else {
                    Err(Self::Error::InvalidNextState)
                }
            }
            _ => Err(Self::Error::InvalidNextState),
        }
    }
}
