use bevy::prelude::{Res, ResMut};
use bevy_tokio_runtime::TokioRuntime;
use tokio::net::TcpListener;

use crate::{
    network_connection_manager::{events::ConnectionEvent, resources::ListenerTask},
    network_settings::NetworkSettings,
};

pub fn set_listener_system(
    tokio_runtime: Res<TokioRuntime>,
    network_settings: Res<NetworkSettings>,
    mut listener_task: ResMut<ListenerTask>,
) {
    let server_address = network_settings.server_address;
    let use_nagle_algorithm = network_settings.use_nagle_algorithm;

    listener_task.task = tokio_runtime.spawn_task(async move {
        let listener = TcpListener::bind(server_address)
            .await
            .expect("Error on bind TcpListener.");

        loop {
            let connection = listener.accept().await;

            let connection_event = match connection {
                Ok((stream, address)) => {
                    if let Err(_) = stream.set_nodelay(!use_nagle_algorithm) {
                        // Could not set nodelay.
                    };

                    ConnectionEvent::Success((stream, address))
                }
                Err(err) => ConnectionEvent::Failure(err),
            };
        }
    });
}
