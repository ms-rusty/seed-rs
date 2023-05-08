use bevy::prelude::{App, Plugin};

use crate::network_listener::NetworkListenerPlugin;

pub struct NetworkServerPlugin;

impl Plugin for NetworkServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NetworkListenerPlugin);

        app.add_startup_system(|| println!("Starting network server plugin."));
    }
}
