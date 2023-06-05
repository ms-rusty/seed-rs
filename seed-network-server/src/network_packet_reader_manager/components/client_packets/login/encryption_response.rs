use bevy::prelude::Component;
use bytes::Bytes;
use seed_network_server_common::VarInt;

use crate::shared::{PacketError, PacketReader};

#[derive(Component)]
pub struct ClientEncryptionResponsePacketId;

#[derive(Debug)]
pub struct ClientEncryptionResponsePacket {
    pub shared_secret_length: VarInt,
    pub shared_secret: Vec<u8>, // byte array
    pub verify_token_length: VarInt,
    pub verify_token: Vec<u8>, // byte array
}

impl TryFrom<&Bytes> for ClientEncryptionResponsePacket {
    type Error = PacketError;

    fn try_from(packet: &Bytes) -> Result<Self, Self::Error> {
        let mut reader = PacketReader::from(packet);
        let shared_secret_length = reader.read_var_int()?;
        let shared_secret = reader.read_fixed_length_bytes(shared_secret_length.value as usize)?;
        let verify_token_length = reader.read_var_int()?;
        let verify_token = reader.read_fixed_length_bytes(verify_token_length.value as usize)?;

        Ok(Self {
            shared_secret_length,
            shared_secret: shared_secret.to_owned(),
            verify_token_length,
            verify_token: verify_token.to_owned(),
        })
    }
}
