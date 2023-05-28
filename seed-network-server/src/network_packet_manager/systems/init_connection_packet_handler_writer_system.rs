use bevy::prelude::{Commands, Entity, Query, Res, With, Without};
use bevy_tokio_runtime::TokioRuntime;

use crate::shared::{
    Connection, ConnectionHandshakingState, ConnectionPacketHandlerWriter, ConnectionStreamWriter,
};

pub fn init_connection_packet_handler_writer_system(
    mut commands: Commands,
    tokio_runtime: Res<TokioRuntime>,
    connection_query: Query<
        (Entity, &ConnectionStreamWriter),
        (
            With<Connection>,
            With<ConnectionHandshakingState>,
            Without<ConnectionPacketHandlerWriter>,
        ),
    >,
) {
    for (entity, stream_writer) in connection_query.iter() {
        let stream_writer = stream_writer.writer.clone();

        let task = tokio_runtime.spawn_task(async move {
            loop {
                let mut stream_writer = stream_writer.lock().await;
                //
            }
        });

        commands
            .entity(entity)
            .insert(ConnectionPacketHandlerWriter::new(task));
    }
}
