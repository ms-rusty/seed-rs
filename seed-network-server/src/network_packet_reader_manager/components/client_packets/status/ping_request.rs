use bevy::prelude::Component;
use bytes::Bytes;

use crate::{network_packet_reader_manager::utils::PacketReader, shared::PacketError};

#[derive(Component)]
pub struct ClientPingRequestPacketId;

#[derive(Component, Debug)]
pub struct ClientPingRequestPacket {
    pub payload: i64,
}

impl TryFrom<&Bytes> for ClientPingRequestPacket {
    type Error = PacketError;

    fn try_from(packet: &Bytes) -> Result<Self, Self::Error> {
        let mut reader = PacketReader::from(packet);
        let payload = reader.read_int(reader.remaining())?;

        Ok(Self { payload })
    }
}
