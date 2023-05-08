use bevy::prelude::{App, Plugin};

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(|| println!("Starting game world plugin."));
    }
}
