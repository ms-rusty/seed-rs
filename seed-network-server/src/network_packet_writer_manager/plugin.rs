use bevy::prelude::{in_state, App, IntoSystemConfigs, Plugin, Update};
use seed_network_server_common::NetworkServerState;

use super::systems::start_connection_packet_writer_system;

pub struct NetworkPacketWriterManagerPlugin;

impl Plugin for NetworkPacketWriterManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (start_connection_packet_writer_system,)
                .chain()
                .run_if(in_state(NetworkServerState::Running)),
        );
    }
}
