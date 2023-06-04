use bevy::prelude::{in_state, App, IntoSystemConfigs, Plugin, Update};
use seed_network_server_common::NetworkServerState;

use super::systems::{
    handshake_message_mapping_system, handshaking_packets_mapping_system,
    insert_packets_into_connection_system, login_packets_mapping_system,
    login_start_message_mapping_system, ping_request_message_mapping_system,
    play_packets_mapping_system, start_connection_packet_reader_system,
    status_packets_mapping_system, status_request_message_mapping_system,
};

pub struct NetworkPacketReaderManagerPlugin;

impl Plugin for NetworkPacketReaderManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_connection_packet_reader_system,
                insert_packets_into_connection_system,
                (
                    (
                        handshaking_packets_mapping_system,
                        status_packets_mapping_system,
                        login_packets_mapping_system,
                        play_packets_mapping_system,
                    ),
                    (
                        // handshaking
                        handshake_message_mapping_system,
                        // status
                        (
                            status_request_message_mapping_system,
                            ping_request_message_mapping_system,
                        ),
                        // login
                        (login_start_message_mapping_system,),
                        // play
                    ),
                )
                    .chain(),
            )
                .chain()
                .run_if(in_state(NetworkServerState::Running)),
        );
    }
}
