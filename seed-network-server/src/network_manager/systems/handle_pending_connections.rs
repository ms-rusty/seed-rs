use bevy::{
    prelude::{
        error, Commands, DespawnRecursiveExt, Entity, EventReader, EventWriter, Query, Res, ResMut,
        With, Without,
    },
    utils::Uuid,
};
use bevy_tokio_runtime::TokioRuntime;

use crate::network_manager::{
    events::{
        shutdown, Client, ClientId, Connection, ConnectionBundle, ConnectionEvent,
        ConnectionHandshakingState, ConnectionSocketAddr, ConnectionStreamReader,
        ConnectionStreamWriter, PacketHandler,
    },
    packets::read_packet,
    resources::{NetworkChannels, NetworkManager},
};

pub fn handle_connection_event_system(
    mut commands: Commands,
    network_channels: Res<NetworkChannels>,
) {
    for connection_event in network_channels
        .pending_connection_channel
        .receiver
        .try_iter()
    {
        match connection_event {
            ConnectionEvent::Success((stream, address)) => {
                let (read, write) = stream.into_split();

                commands.spawn((
                    ConnectionBundle {
                        connection: Connection,
                        stream_reader: ConnectionStreamReader::new(read),
                        stream_writer: ConnectionStreamWriter::new(write),
                        address: ConnectionSocketAddr::new(address),
                    },
                    ConnectionHandshakingState,
                ));
            }
            ConnectionEvent::Failure(_) => {}
        }
    }
}

pub fn create_packet_handlers_system(
    mut commands: Commands,
    tokio_runtime: Res<TokioRuntime>,
    network_channels: Res<NetworkChannels>,
    connection_query: Query<(Entity, &ConnectionStreamReader), Without<PacketHandler>>,
) {
    for (entity, stream_reader) in connection_query.iter() {
        let pending_client_packet_channel_sender = network_channels
            .pending_client_packet_channel
            .sender
            .clone();

        let stream_reader = stream_reader.reader.clone();

        let packet_handler = tokio_runtime.spawn_task(async move {
            loop {
                let mut stream_reader = stream_reader.lock().await;
                let packet = match read_packet(&mut stream_reader).await {
                    Ok(packet) => packet,
                    Err(err) => {
                        // Error on read packet.
                        error!("Error on read packet. {}", err);
                        break;
                    }
                };

                if let Err(_) = pending_client_packet_channel_sender.send((entity, packet)) {
                    // Error on send read packet.
                    println!("Error on send read packet.");
                }
            }
        });

        commands
            .entity(entity)
            .insert(PacketHandler::new(packet_handler));
    }
}
