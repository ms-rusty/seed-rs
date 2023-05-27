use bevy::prelude::{App, OnEnter, Plugin};
use seed_network_server_common::NetworkServerState;

use super::{resources::ConnectionChannel, systems::accept_incoming_connections_system};

pub struct NetworkConnectionManagerPlugin;

impl Plugin for NetworkConnectionManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConnectionChannel>();

        app.add_systems(
            OnEnter(NetworkServerState::AcceptConnections),
            accept_incoming_connections_system,
        );
    }
}
