use bevy::prelude::{Res, ResMut};
use bevy_tokio_runtime::TokioRuntime;
use tokio::net::TcpListener;

use crate::{
    network_manager::{
        events::{Connection, ConnectionEvent},
        resources::{NetworkChannels, NetworkManager},
    },
    network_settings::NetworkSettings,
};

pub fn start_listening_system(
    // mut commands: Commands,
    tokio_runtime: Res<TokioRuntime>,
    network_channels: Res<NetworkChannels>,
    mut network_manager: ResMut<NetworkManager>,
    network_settings: Res<NetworkSettings>,
) {
    if network_manager.listener_handler.is_some() {
        return;
    }

    let use_nagle_algorithm = network_settings.nagle_algorithm;

    // let (response_listening_sender, response_listening_receiver) = crossbeam_channel::bounded::<()>(1);

    // commands.insert_resource(Listening(response_listening_receiver));

    let pending_connection_channel_sender =
        network_channels.pending_connection_channel.sender.clone();

    network_manager.listener_handler = Some(tokio_runtime.spawn_task(async move {
        let listener = TcpListener::bind("127.0.0.1:65535").await;
        let listener = match listener {
            Ok(listener) => listener,
            Err(err) => panic!("Error on bind Tokio TcpListener. {:?}", err),
        };

        // response_listening_sender.send(()).ok();

        loop {
            let connection = listener.accept().await;

            let connection_event = match connection {
                Ok((stream, address)) => {
                    if let Err(_) = stream.set_nodelay(!use_nagle_algorithm) {
                        // Could not set nodelay.
                    };

                    let connection = Connection::new(address, stream);
                    ConnectionEvent::Success(connection)
                }
                Err(err) => ConnectionEvent::Failure(err),
            };

            if let Err(_) = pending_connection_channel_sender.send(connection_event) {
                // Error on send connection event: channel
            }
        }
    }));
}
