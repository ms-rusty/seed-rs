use bevy::prelude::{Res, ResMut};
use bevy_tokio_runtime::TokioRuntime;
use tokio::net::TcpListener;

use crate::{network_listener_manager::resources::ListenerTask, network_settings::NetworkSettings};

pub fn create_listener_system(
    tokio_runtime: Res<TokioRuntime>,
    network_settings: Res<NetworkSettings>,
    mut listener_task: ResMut<ListenerTask>,
) {
    let server_address = network_settings.server_address;

    listener_task.task =
        tokio_runtime.spawn_task(async move { TcpListener::bind(server_address).await });
}
