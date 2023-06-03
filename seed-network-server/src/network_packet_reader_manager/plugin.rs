use bevy::prelude::{in_state, App, IntoSystemConfigs, Plugin, Update};
use seed_network_server_common::NetworkServerState;

use super::systems::{
    insert_packets_into_connection_system, start_connection_packet_reader_system,
};

pub struct NetworkPacketReaderManagerPlugin;

impl Plugin for NetworkPacketReaderManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_connection_packet_reader_system,
                insert_packets_into_connection_system,
            )
                .chain()
                .run_if(in_state(NetworkServerState::Running)),
        );
    }
}
