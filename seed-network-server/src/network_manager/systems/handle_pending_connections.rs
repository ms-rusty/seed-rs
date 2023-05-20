use bevy::{
    prelude::{error, Res, ResMut},
    utils::Uuid,
};
use bevy_tokio_runtime::TokioRuntime;

use crate::network_manager::{
    events::{read_packet, shutdown, Client, ClientId, ConnectionEvent},
    resources::{NetworkChannels, NetworkManager},
};

pub fn handle_pending_connections_system(
    // mut commands: Commands,
    tokio_runtime: Res<TokioRuntime>,
    mut network_manager: ResMut<NetworkManager>,
    network_channels: Res<NetworkChannels>,
    // network_settings: Res<NetworkSettings>,
) {
    for connection_event in network_channels
        .pending_connection_channel
        .receiver
        .try_iter()
    {
        match connection_event {
            ConnectionEvent::Success(connection) => {
                let pending_client_packet_channel_sender = network_channels
                    .pending_client_packet_channel
                    .sender
                    .clone();

                let connection_stream_reader = connection.stream.reader.clone();
                let connection_stream_writer = connection.stream.writer.clone();

                let client_id = ClientId(Uuid::new_v4());

                let client_packet_handler = tokio_runtime.spawn_task(async move {
                    loop {
                        let mut connection_stream_reader = connection_stream_reader.lock().await;
                        let packet = match read_packet(&mut connection_stream_reader).await {
                            Ok(packet) => packet,
                            Err(err) => {
                                // Error on read packet.
                                error!("Error on read packet. {}", err);
                                break;
                            }
                        };

                        if let Err(_) =
                            pending_client_packet_channel_sender.send((client_id, packet))
                        {
                            // Error on send read packet.
                            println!("Error on send read packet.");
                        }
                    }

                    let mut connection_stream_writer = connection_stream_writer.lock().await;
                    match shutdown(&mut connection_stream_writer).await {
                        Ok(_) => println!("shutdown..."),
                        Err(err) => println!("error on shutdown... {}", err),
                    }
                });

                let client = Client::new(connection, client_packet_handler);
                network_manager.clients.insert(client_id, client);
            }
            ConnectionEvent::Failure(_) => {
                // Failure on connection.
            }
        }
    }
}
