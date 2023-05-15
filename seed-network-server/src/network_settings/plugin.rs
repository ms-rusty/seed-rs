use bevy::prelude::{App, Plugin, PreStartup, Startup};

use super::{
    resources::NetworkSettings,
    systems::{create_settings_system, load_settings_system},
};

pub struct NetworkSettingsPlugin;

impl Plugin for NetworkSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkSettings>();

        app.add_systems(PreStartup, create_settings_system);
        app.add_systems(Startup, load_settings_system);
    }
}
