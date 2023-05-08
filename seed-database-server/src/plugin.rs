use bevy::prelude::{App, Plugin};

pub struct DatabaseServerPlugin;

impl Plugin for DatabaseServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(|| println!("Starting database server plugin."));
    }
}
