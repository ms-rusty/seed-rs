use bevy::prelude::Bundle;

use crate::shared::components::{Connection, ConnectionStreamReader, ConnectionStreamWriter};

#[derive(Bundle)]
pub struct ConnectionBundle {
    pub connection: Connection,
    pub stream_reader: ConnectionStreamReader,
    pub stream_writer: ConnectionStreamWriter,
}
