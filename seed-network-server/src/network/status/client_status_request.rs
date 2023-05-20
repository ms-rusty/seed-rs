use crate::network::{packet::PacketError, ClientStatusPackets, Packet};

#[derive(Debug)]
pub struct PacketClientStatusRequest {}

impl TryFrom<&Packet> for PacketClientStatusRequest {
    type Error = PacketError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        if packet.command != ClientStatusPackets::StatusRequest as i32 {
            return Err(PacketError::InvalidPacket(format!(
                "Received Command: {}, Client Packet: ClientStatusPackets::StatusRequest",
                packet.command
            )));
        }

        Ok(Self {})
    }
}
