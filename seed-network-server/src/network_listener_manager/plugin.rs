use bevy::prelude::{
    in_state, resource_exists, App, IntoSystem, IntoSystemConfigs, OnEnter, Plugin, Update,
};
use seed_common::handle_error;
use seed_network_server_common::NetworkServerState;

use crate::shared::NetworkListener;

use super::{
    resources::ListenerChannel,
    systems::{load_listener_system, next_state_system, start_listener_system},
};

pub struct NetworkListenerManagerPlugin;

impl Plugin for NetworkListenerManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ListenerChannel>();

        app.add_systems(
            OnEnter(NetworkServerState::StartingListener),
            start_listener_system,
        );

        app.add_systems(Update, load_listener_system.pipe(handle_error));

        app.add_systems(
            Update,
            next_state_system
                .run_if(resource_exists::<NetworkListener>())
                .run_if(in_state(NetworkServerState::StartingListener)),
        );
    }
}
