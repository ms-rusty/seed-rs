use bevy::prelude::{App, IntoSystem, IntoSystemConfigs, OnEnter, Plugin};
use seed_common::handle_error;

use super::systems::{next_state_system, set_runtime_system};

use seed_network_server_common::NetworkServerState;

pub struct NetworkRuntimePlugin;

impl Plugin for NetworkRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(NetworkServerState::SettingRuntime),
            (set_runtime_system.pipe(handle_error), next_state_system).chain(),
        );
    }
}
