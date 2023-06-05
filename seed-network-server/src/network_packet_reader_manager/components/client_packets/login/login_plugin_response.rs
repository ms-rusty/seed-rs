use bevy::prelude::Component;
use bytes::Bytes;
use seed_network_server_common::VarInt;

use crate::shared::{PacketError, PacketReader};

#[derive(Component)]
pub struct ClientLoginPluginResponsePacketId;

#[derive(Debug)]
pub struct ClientLoginPluginResponsePacket {
    pub message_id: VarInt,
    pub successful: bool,
    // data length is 1048576 bytes.
    pub data: Option<Vec<u8>>, // byte array
}

impl TryFrom<&Bytes> for ClientLoginPluginResponsePacket {
    type Error = PacketError;

    fn try_from(packet: &Bytes) -> Result<Self, Self::Error> {
        let mut reader = PacketReader::from(packet);
        let message_id = reader.read_var_int()?;
        let successful = reader.read_bool()?;

        let data = if reader.has_remaining() {
            let remaining_bytes = reader.get_remaining_bytes()?;
            Some(remaining_bytes.to_owned())
        } else {
            None
        };

        Ok(Self {
            message_id,
            successful,
            data,
        })
    }
}
