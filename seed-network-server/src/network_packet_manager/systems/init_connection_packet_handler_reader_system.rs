use bevy::prelude::{error, Commands, Entity, Query, Res, With, Without};
use bevy_tokio_runtime::TokioRuntime;
use bytes::BytesMut;
use tokio::io::AsyncReadExt;

use crate::{
    network_packet_manager::{
        components::Packet, events::ReadPacketEvent, resources::ReadPacketChannel,
    },
    shared::{
        Connection, ConnectionHandshakingState, ConnectionPacketHandlerReader,
        ConnectionStreamReader,
    },
};

pub fn init_connection_packet_handler_reader_system(
    mut commands: Commands,
    tokio_runtime: Res<TokioRuntime>,
    read_packet_channel: Res<ReadPacketChannel>,
    connection_query: Query<
        (Entity, &ConnectionStreamReader),
        (
            With<Connection>,
            With<ConnectionHandshakingState>,
            Without<ConnectionPacketHandlerReader>,
        ),
    >,
) {
    for (entity, stream_reader) in connection_query.iter() {
        let stream_reader = stream_reader.reader.clone();
        let packet_channel_sender = read_packet_channel.sender.clone();

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

                let read_packet_event = match Packet::try_from(buffer) {
                    Ok(packet) => ReadPacketEvent::Success((entity, packet)),
                    Err(err) => ReadPacketEvent::Failure((entity, err)),
                };

                println!("{:?}", read_packet_event);

                if let Err(_) = packet_channel_sender.send(read_packet_event) {
                    // Error on send packet event.
                    break;
                };
            }
        });

        commands
            .entity(entity)
            .insert(ConnectionPacketHandlerReader::new(task));
    }
}
