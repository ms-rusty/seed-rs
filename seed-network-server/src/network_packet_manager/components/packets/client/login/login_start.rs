use bevy::prelude::Component;
use uuid::Uuid;

use crate::network_packet_manager::components::packets::{
    packet_errors::PacketError, packet_reader::PacketReader, Packet,
};

use super::packets_id::ClientLoginPackets;

#[derive(Component, Debug)]
pub struct ClientLoginStartPacket<'packet> {
    pub username: &'packet str,
    pub has_player_uuid: bool,
    pub player_uuid: Option<Uuid>,
}

impl<'packet> TryFrom<&'packet Packet> for ClientLoginStartPacket<'packet> {
    type Error = PacketError;

    fn try_from(packet: &'packet Packet) -> Result<Self, Self::Error> {
        if packet.id != ClientLoginPackets::LoginStart {
            return Err(Self::Error::InvalidPacket);
        }

        let mut reader = PacketReader::from(packet);
        let username = reader.read_str()?;
        let has_player_uuid = reader.read_bool()?;
        let player_uuid = if has_player_uuid {
            Some(reader.read_uuid()?)
        } else {
            None
        };

        Ok(Self {
            username,
            has_player_uuid,
            player_uuid,
        })
    }
}
