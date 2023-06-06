use bevy::prelude::{in_state, App, IntoSystemConfigs, Last, Plugin, Update};
use seed_network_server_common::NetworkServerState;

use super::systems::{
    ping_response_message_mapping_system, ping_response_packets_mapping_system,
    start_connection_packet_writer_system, status_response_message_mapping_system,
    status_response_packets_mapping_system,
};

pub struct NetworkPacketWriterManagerPlugin;

impl Plugin for NetworkPacketWriterManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Last,
            (
                (
                    status_response_message_mapping_system,
                    ping_response_message_mapping_system,
                ),
                (
                    status_response_packets_mapping_system,
                    ping_response_packets_mapping_system,
                ),
                start_connection_packet_writer_system,
            )
                .chain()
                .run_if(in_state(NetworkServerState::Running)),
        );
    }
}
