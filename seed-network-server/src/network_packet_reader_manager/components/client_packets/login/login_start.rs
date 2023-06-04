use bevy::prelude::Component;
use bytes::Bytes;
use uuid::Uuid;

use crate::shared::{PacketError, PacketReader};

#[derive(Component)]
pub struct ClientLoginStartPacketId;

#[derive(Debug)]
pub struct ClientLoginStartPacket {
    pub username: String,
    pub has_player_uuid: bool,
    pub player_uuid: Option<Uuid>,
}

impl TryFrom<&Bytes> for ClientLoginStartPacket {
    type Error = PacketError;

    fn try_from(packet: &Bytes) -> Result<Self, Self::Error> {
        let mut reader = PacketReader::from(packet);
        let username = reader.read_str()?;
        let has_player_uuid = reader.read_bool()?;
        let player_uuid = if has_player_uuid {
            Some(reader.read_uuid()?)
        } else {
            None
        };

        let username = username.to_owned();

        Ok(Self {
            username,
            has_player_uuid,
            player_uuid,
        })
    }
}
