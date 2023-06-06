use bevy::prelude::Component;
use bytes::Bytes;
use seed_network_server_common::VarInt;

use crate::shared::{PacketError, PacketReader};

#[derive(Component)]
pub struct ClientEncryptionResponsePacketId;

#[derive(Debug)]
pub struct ClientEncryptionResponsePacket<'packet> {
    pub shared_secret_length: VarInt,
    pub shared_secret: &'packet [u8], // byte array
    pub verify_token_length: VarInt,
    pub verify_token: &'packet [u8], // byte array
}

impl<'packet> TryFrom<&'packet Bytes> for ClientEncryptionResponsePacket<'packet> {
    type Error = PacketError;

    fn try_from(packet: &'packet Bytes) -> Result<Self, Self::Error> {
        let mut reader = PacketReader::from(packet);
        let shared_secret_length = reader.read_var_int()?;
        let shared_secret = reader.read_fixed_length_bytes(shared_secret_length.value as usize)?;
        let verify_token_length = reader.read_var_int()?;
        let verify_token = reader.read_fixed_length_bytes(verify_token_length.value as usize)?;

        Ok(Self {
            shared_secret_length,
            shared_secret,
            verify_token_length,
            verify_token,
        })
    }
}
