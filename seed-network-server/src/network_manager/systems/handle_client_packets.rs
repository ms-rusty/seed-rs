use bevy::prelude::Res;
use bevy_tokio_runtime::TokioRuntime;

use crate::network_manager::resources::{NetworkChannels, NetworkManager};

pub fn handle_client_packets(
    network_channels: Res<NetworkChannels>,
    tokio_runtie: Res<TokioRuntime>,
    network_manager: Res<NetworkManager>,
) {
    for packet in network_channels
        .pending_client_packet_channel
        .receiver
        .try_iter()
    {
        tokio_runtie.spawn_task(async move {});
    }
}
