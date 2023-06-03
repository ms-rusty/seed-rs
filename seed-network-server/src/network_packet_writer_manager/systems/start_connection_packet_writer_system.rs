use bevy::prelude::{Commands, Entity, Query, Res, With, Without};
use bevy_tokio_runtime::TokioRuntime;

use crate::{
    network_packet_writer_manager::components::ConnectionPacketWriter,
    shared::{Connection, ConnectionServerPacketsChannel, ConnectionStreamWriter},
};

pub fn start_connection_packet_writer_system(
    mut commands: Commands,
    tokio_runtime: Res<TokioRuntime>,
    connection_query: Query<
        (
            Entity,
            &ConnectionStreamWriter,
            &ConnectionServerPacketsChannel,
        ),
        (With<Connection>, Without<ConnectionPacketWriter>),
    >,
) {
    for (entity, stream_writer, packets_channel) in connection_query.iter() {
        let stream_writer = stream_writer.writer.clone();
        let packet_channel_receiver = packets_channel.receiver.clone();

        let task = tokio_runtime.spawn_task(async move {
            loop {
                while let Ok(packet) = packet_channel_receiver.try_recv() {
                    let mut stream_writer = stream_writer.lock().await;
                }
            }
        });

        commands
            .entity(entity)
            .insert(ConnectionPacketWriter::new(task));
    }
}
