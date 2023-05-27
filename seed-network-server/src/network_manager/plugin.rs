use bevy::prelude::{App, First, IntoSystemConfigs, Plugin, PreUpdate, Res, ResMut, Startup};
use bevy_tokio_runtime::TokioRuntime;
use tokio::runtime::Builder;

use crate::network_settings::NetworkSettings;

use super::{
    resources::{NetworkChannels, NetworkManager},
    systems::{
        create_packet_handlers_system, handle_client_packets, handle_connection_event_system,
        start_listening_system,
    },
};

pub struct NetworkManagerPlugin;

impl Plugin for NetworkManagerPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<NetworkManager>();
        // app.init_resource::<NetworkChannels>();

        // app.add_systems(Startup, (start_listening_system).chain());

        // app.add_systems(
        //     First,
        //     (
        //         handle_connection_event_system,
        //         create_packet_handlers_system,
        //     )
        //         .chain(),
        // );
        // app.add_systems(PreUpdate, (handle_client_packets).chain());
    }
}
