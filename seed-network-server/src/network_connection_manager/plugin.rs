use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, Plugin, PreUpdate};
use seed_network_server_common::NetworkServerState;

use super::{
    resources::ConnectionChannel,
    systems::{
        accept_incoming_connections_system, handle_new_connections_events_system, next_state_system,
    },
};

pub struct NetworkConnectionManagerPlugin;

impl Plugin for NetworkConnectionManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConnectionChannel>();

        app.add_systems(
            OnEnter(NetworkServerState::AcceptConnections),
            (accept_incoming_connections_system, next_state_system).chain(),
        );

        app.add_systems(
            PreUpdate,
            handle_new_connections_events_system.run_if(in_state(NetworkServerState::Running)),
        );
    }
}
