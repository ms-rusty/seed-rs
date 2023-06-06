use bevy::prelude::{error, Commands, Entity, Query, Res, With, Without};
use bevy_tokio_runtime::TokioRuntime;
use bytes::{BufMut, BytesMut};
use tokio::io::AsyncWriteExt;

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
                let mut stream_writer = stream_writer.lock().await;
                match packet_channel_receiver.try_recv() {
                    Ok(packet) => {
                        let packet_id = packet.get_id();
                        let packet_data = packet.get_data();

                        let mut packet_length = (packet_id.position + packet_data.len()) as i32;

                        let mut buffer = BytesMut::with_capacity(1024 * 1024);

                        loop {
                            let mut byte = (packet_length & 0x7F) as u8;
                            packet_length >>= 7;

                            if packet_length != 0 {
                                byte |= 0x80;
                            }

                            buffer.put_u8(byte);

                            if packet_length == 0 {
                                break;
                            }
                        }

                        let mut value = packet_id.value as i32;

                        loop {
                            let mut byte = (value & 0x7F) as u8;
                            value >>= 7;

                            if value != 0 {
                                byte |= 0x80;
                            }

                            buffer.put_u8(byte);

                            if value == 0 {
                                break;
                            }
                        }

                        buffer.put(packet_data);

                        if let Err(err) = stream_writer.write_all(&buffer).await {
                            error!("{:?}", err);
                        }

                        if let Err(err) = stream_writer.flush().await {
                            error!("{:?}", err);
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        commands
            .entity(entity)
            .insert(ConnectionPacketWriter::new(task));
    }
}
