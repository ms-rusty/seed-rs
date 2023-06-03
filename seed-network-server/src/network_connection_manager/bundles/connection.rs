use bevy::prelude::Bundle;

use crate::shared::{
    Connection, ConnectionClientPacketsChannel, ConnectionServerPacketsChannel,
    ConnectionStreamReader, ConnectionStreamWriter,
};

#[derive(Bundle)]
pub struct ConnectionBundle {
    pub connection: Connection,
    pub stream_reader: ConnectionStreamReader,
    pub stream_writer: ConnectionStreamWriter,
    pub client_packets_channel: ConnectionClientPacketsChannel,
    pub server_packets_channel: ConnectionServerPacketsChannel,
}
