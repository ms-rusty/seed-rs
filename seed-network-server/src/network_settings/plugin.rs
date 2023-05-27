use bevy::prelude::{App, IntoSystem, IntoSystemConfigs, OnEnter, Plugin};
use seed_common::handle_error;
use seed_network_server_common::NetworkServerState;

use super::{
    resources::NetworkSettings,
    systems::{load_settings_system, next_state_system},
};

pub struct NetworkSettingsPlugin;

impl Plugin for NetworkSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkSettings>();

        app.add_systems(
            OnEnter(NetworkServerState::LoadingSettings),
            (load_settings_system.pipe(handle_error), next_state_system).chain(),
        );
    }
}
