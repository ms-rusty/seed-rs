use bevy::prelude::{App, IntoSystem, IntoSystemConfigs, OnEnter, Plugin};
use seed_common::handle_error;

use super::systems::{next_state, set_runtime};

use seed_network_server_common::NetworkServerState;

pub struct NetworkRuntimePlugin;

impl Plugin for NetworkRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(NetworkServerState::SettingRuntime),
            (set_runtime.pipe(handle_error), next_state).chain(),
        );
    }
}
