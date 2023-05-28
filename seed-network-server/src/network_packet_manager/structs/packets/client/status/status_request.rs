use crate::network_packet_manager::packets::{
    packet_errors::PacketError, ClientStatusPackets, Packet,
};

#[derive(Debug)]
pub struct ClientStatusRequestPacket;

impl TryFrom<&Packet> for ClientStatusRequestPacket {
    type Error = PacketError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        if packet.id != ClientStatusPackets::StatusRequest {
            return Err(Self::Error::InvalidPacket);
        }

        Ok(Self)
    }
}
