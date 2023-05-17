use std::sync::Arc;

use bevy::{
    prelude::{Res, ResMut},
    utils::Uuid,
};
use bevy_tokio_runtime::TokioRuntime;
use tokio::sync::Mutex;

use crate::network_manager::{
    events::{Client, ClientId, ConnectionEvent},
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
                let (t, _) = crossbeam_channel::unbounded::<()>();

                let connection = Arc::new(Mutex::new(connection));
                let client_id = ClientId(Uuid::new_v4());
                let client_connection = connection.clone();

                let client_packet_handler = tokio_runtime.spawn_task(async move {
                    loop {
                        let mut client_connection = client_connection.lock().await;
                        let Ok(packet) = client_connection.read_packet().await else {
                            // Error on read packet.
                            break;
                        };

                        if let Err(_) = t.send(packet) {
                            // Error on send read packet.
                        }
                    }

                    let mut client_connection = client_connection.lock().await;
                    match client_connection.shutdown().await {
                        Ok(_) => todo!(),
                        Err(_) => todo!(),
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
