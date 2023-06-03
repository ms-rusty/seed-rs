use bevy::prelude::{error, Commands, Entity, Query, Res, With, Without};
use bevy_tokio_runtime::TokioRuntime;
use bytes::BytesMut;
use tokio::io::AsyncReadExt;

use crate::{
    network_packet_reader_manager::components::ConnectionPacketReader,
    shared::{Connection, ConnectionClientPacketsChannel, ConnectionStreamReader, Packet},
};

pub fn start_connection_packet_reader_system(
    mut commands: Commands,
    tokio_runtime: Res<TokioRuntime>,
    connection_query: Query<
        (
            Entity,
            &ConnectionStreamReader,
            &ConnectionClientPacketsChannel,
        ),
        (With<Connection>, Without<ConnectionPacketReader>),
    >,
) {
    for (entity, stream_reader, packets_channel) in connection_query.iter() {
        let stream_reader = stream_reader.reader.clone();
        let packets_channel_sender = packets_channel.sender.clone();

        let task = tokio_runtime.spawn_task(async move {
            loop {
                let mut stream_reader = stream_reader.lock().await;
                let mut buffer = BytesMut::with_capacity(4 * 1024);
                if let Err(_) = stream_reader.read_buf(&mut buffer).await {
                    // Error on read buffer.
                    break;
                }

                if buffer.is_empty() {
                    error!("Lost connection.");
                    break;
                }

                let packet = Packet::try_from(buffer);
                if let Err(_) = packets_channel_sender.send(packet) {
                    // Error on send packet event.
                    break;
                };
            }
        });

        commands
            .entity(entity)
            .insert(ConnectionPacketReader::new(task));
    }
}
