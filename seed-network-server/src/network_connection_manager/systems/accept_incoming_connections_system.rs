use bevy::prelude::{Res, ResMut};
use bevy_tokio_runtime::TokioRuntime;

use crate::{
    network_connection_manager::{events::ConnectionEvent, resources::ConnectionChannel},
    network_settings::NetworkSettings,
    shared::NetworkListener,
};

pub fn accept_incoming_connections_system(
    tokio_runtime: Res<TokioRuntime>,
    network_settings: Res<NetworkSettings>,
    mut network_listener: ResMut<NetworkListener>,
    connection_channel: Res<ConnectionChannel>,
) {
    let server_socket = network_listener.server_socket.clone();
    let use_nagle_algorithm = network_settings.use_nagle_algorithm;

    let connection_channel_sender = connection_channel.sender.clone();

    let task = tokio_runtime.spawn_task(async move {
        let server_socket = server_socket.lock().await;
        loop {
            let connection = server_socket.accept().await;

            let connection_event = match connection {
                Ok((stream, address)) => {
                    if let Err(_) = stream.set_nodelay(!use_nagle_algorithm) {
                        // Could not set nodelay.
                    };

                    ConnectionEvent::Success((stream, address))
                }
                Err(err) => ConnectionEvent::Failure(err),
            };

            if let Err(_) = connection_channel_sender.send(connection_event) {
                // Error on send connection event: channel
                break;
            }
        }
    });

    network_listener.accept_connection_task = Some(task);
}
