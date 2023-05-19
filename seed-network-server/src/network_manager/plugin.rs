use bevy::prelude::{
    App, IntoSystemConfigs, Plugin, PostStartup, PreStartup, PreUpdate, Res, ResMut, Startup,
};
use bevy_tokio_runtime::TokioRuntime;
use tokio::runtime::Builder;

use super::{
    resources::{NetworkChannels, NetworkManager},
    systems::{handle_client_packets, handle_pending_connections_system, start_listening_system},
};

pub struct NetworkManagerPlugin;

impl Plugin for NetworkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkManager>();
        app.init_resource::<NetworkChannels>();

        app.add_systems(
            Startup,
            (configure_tokio_runtime_system, start_listening_system).chain(),
        );

        app.add_systems(
            PreUpdate,
            (handle_pending_connections_system, handle_client_packets).chain(),
        );
    }
}

fn configure_tokio_runtime_system(mut tokio_runtime: ResMut<TokioRuntime>) {
    let runtime = Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .worker_threads(2)
        .build()
        .expect("Error on build Tokio Runtime.");

    tokio_runtime.new(runtime);
}
