use bevy::prelude::{App, Plugin};

pub struct NetworkListenerPlugin;

impl Plugin for NetworkListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(|| println!("Starting network listener plugin."));
    }
}
