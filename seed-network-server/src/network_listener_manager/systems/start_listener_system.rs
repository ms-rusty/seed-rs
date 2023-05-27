use bevy::prelude::{error, Res};
use bevy_tokio_runtime::TokioRuntime;
use tokio::net::TcpListener;

use crate::{
    network_listener_manager::resources::ListenerChannel, network_settings::NetworkSettings,
};

pub fn start_listener_system(
    tokio_runtime: Res<TokioRuntime>,
    network_settings: Res<NetworkSettings>,
    listener_channel: Res<ListenerChannel>,
) {
    let server_address = network_settings.server_address;
    let listener_channel_sender = listener_channel.sender.clone();

    tokio_runtime.spawn_task(async move {
        if let Err(err) = listener_channel_sender.send(TcpListener::bind(server_address).await) {
            error!("{:?}", err);
        }
    });
}
