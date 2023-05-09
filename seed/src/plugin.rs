use bevy::prelude::{App, AssetPlugin, AssetServer, Plugin, Res};

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_configuration_system);
    }
}

fn load_configuration_system() {}
