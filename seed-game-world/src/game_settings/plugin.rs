use bevy::prelude::{App, Plugin};

use super::resources::GameSettings;

pub struct GameSettingsPlugin;

impl Plugin for GameSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>();

        // app.add_systems(PreStartup, create_settings_system);
        // app.add_systems(Startup, load_settings_system);
    }
}
