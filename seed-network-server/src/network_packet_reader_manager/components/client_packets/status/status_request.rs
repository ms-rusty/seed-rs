use bevy::prelude::Component;
use bytes::Bytes;

use crate::shared::PacketError;

#[derive(Component)]
pub struct ClientStatusRequestPacketId;

#[derive(Component, Debug)]
pub struct ClientStatusRequestPacket;

impl<'packet> TryFrom<&'packet Bytes> for ClientStatusRequestPacket {
    type Error = PacketError;

    fn try_from(_: &'packet Bytes) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}
