use bevy::prelude::{Commands, Res};

use crate::{
    network_connection_manager::{events::ConnectionEvent, resources::ConnectionChannel},
    shared::{
        Connection, ConnectionBundle, ConnectionHandshakingState, ConnectionStreamReader,
        ConnectionStreamWriter,
    },
};

pub fn handle_new_connections_events_system(
    mut commands: Commands,
    connection_channel: Res<ConnectionChannel>,
) {
    for connection_event in connection_channel.receiver.try_iter() {
        match connection_event {
            ConnectionEvent::Success((stream, socket_addr)) => {
                let (read, write) = stream.into_split();

                commands.spawn((
                    ConnectionBundle {
                        connection: Connection::new(socket_addr),
                        stream_reader: ConnectionStreamReader::new(read),
                        stream_writer: ConnectionStreamWriter::new(write),
                    },
                    ConnectionHandshakingState,
                ));
            }
            ConnectionEvent::Failure(_) => todo!(),
        }
    }
}
