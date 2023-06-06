use bevy::prelude::Component;
use bytes::Bytes;
use seed_network_server_common::VarInt;

use crate::shared::{PacketError, PacketReader, PacketReaderError};

#[derive(Component)]
pub struct ClientLoginPluginResponsePacketId;

const MAX_DATA_LENGTH: usize = 1048576;

#[derive(Debug)]
pub struct ClientLoginPluginResponsePacket<'packet> {
    pub message_id: VarInt,
    pub successful: bool,
    pub data: Option<&'packet [u8]>, // byte array
}

impl<'packet> TryFrom<&'packet Bytes> for ClientLoginPluginResponsePacket<'packet> {
    type Error = PacketError;

    fn try_from(packet: &'packet Bytes) -> Result<Self, Self::Error> {
        let mut reader = PacketReader::from(packet);
        let message_id = reader.read_var_int()?;
        let successful = reader.read_bool()?;

        let data = if reader.has_remaining() {
            if reader.remaining() > MAX_DATA_LENGTH {
                return Err(PacketReaderError::ByteArrayTooBig.into());
            }

            Some(reader.get_remaining_bytes()?)
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
