use bevy::prelude::Component;

use crate::network_packet_manager::components::{
    packets::{packet_errors::PacketError, packet_reader::PacketReader},
    Packet,
};

use super::packets_id::ClientStatusPackets;

#[derive(Component, Debug)]
pub struct ClientPingRequestPacket {
    pub payload: i64,
}

impl TryFrom<&Packet> for ClientPingRequestPacket {
    type Error = PacketError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        if packet.id != ClientStatusPackets::PingRequest {
            return Err(Self::Error::InvalidPacket);
        }

        let mut reader = PacketReader::from(packet);
        let payload = reader.read_int(reader.remaining())?;

        Ok(Self { payload })
    }
}
