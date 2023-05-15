use bevy::prelude::{Commands, Res};
use bevy_tokio_runtime::TokioRuntime;

use crate::{
    network_manager::{
        events::{Client, ConnectionEvent},
        resources::{NetworkChannels, NetworkManager},
    },
    network_settings::NetworkSettings,
};

pub fn handle_pending_connections_system(
    mut commands: Commands,
    tokio_runtime: Res<TokioRuntime>,
    network_manager: Res<NetworkManager>,
    network_channels: Res<NetworkChannels>,
    network_settings: Res<NetworkSettings>,
) {
    for connection_event in network_channels
        .pending_connection_channel
        .receiver
        .try_iter()
    {
        if let ConnectionEvent::Success(connection) = connection_event {
            let (client_sender, client_receiver) = crossbeam_channel::unbounded::<()>();

            let client = Client {};

            let client_packet_handler = tokio_runtime.spawn_task(async move {
                loop {
                    match connection.read_packet().await {
                        Ok(_) => todo!(),
                        Err(_) => todo!(),
                    }
                }
            });

            let server_packet_handler = tokio_runtime.spawn_task(async move {
                loop {
                    match connection.write_packet().await {
                        Ok(_) => todo!(),
                        Err(_) => todo!(),
                    }
                }
            });

            network_manager.clients.insert(index, element);
        }

        if let ConnectionEvent::Failure(err) = connection_event {
            // Failure on connection.
        }
    }
}
