use bevy::prelude::{in_state, App, IntoSystemConfigs, Plugin, Update};
use seed_network_server_common::NetworkServerState;

use super::{
    resources::ReadPacketChannel,
    systems::{init_connection_packet_handler_reader_system, map_packets_events_system},
};

pub struct NetworkPacketManagerPlugin;

impl Plugin for NetworkPacketManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReadPacketChannel>();

        app.add_systems(
            Update,
            (
                init_connection_packet_handler_reader_system,
                map_packets_events_system,
            )
                .chain()
                .run_if(in_state(NetworkServerState::Running)),
        );
    }
}
