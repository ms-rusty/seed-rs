use bevy::{
    app::AppExit,
    prelude::{
        error, App, EventWriter, In, IntoSystem, IntoSystemConfigs, OnEnter, Plugin, PreStartup,
    },
};
use seed_common::handle_error;
use seed_network_server_common::NetworkServerState;

use super::{
    resources::NetworkSettings,
    systems::{create_settings_system, load_settings_system, next_state},
};

pub struct NetworkSettingsPlugin;

impl Plugin for NetworkSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkSettings>();

        app.add_systems(
            OnEnter(NetworkServerState::LoadingSettings),
            (
                create_settings_system.pipe(handle_error),
                load_settings_system.pipe(handle_error),
                next_state,
            )
                .chain(),
        );
    }
}
