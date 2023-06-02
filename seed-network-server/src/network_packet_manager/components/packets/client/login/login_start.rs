use bevy::prelude::Component;
use bytes::Bytes;
use uuid::Uuid;

use crate::network_packet_manager::components::packets::{
    packet_errors::PacketError, packet_reader::PacketReader,
};

#[derive(Component)]
pub struct ClientLoginStartPacketId;

#[derive(Component, Debug)]
pub struct ClientLoginStartPacket<'packet> {
    pub username: &'packet str,
    pub has_player_uuid: bool,
    pub player_uuid: Option<Uuid>,
}

impl<'packet> TryFrom<&'packet Bytes> for ClientLoginStartPacket<'packet> {
    type Error = PacketError;

    fn try_from(packet: &'packet Bytes) -> Result<Self, Self::Error> {
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
