use bevy::prelude::{App, Plugin, PostStartup, PreUpdate};

use super::{
    resources::NetworkManager,
    systems::{handle_pending_connections_system, start_listening_system},
};

pub struct NetworkManagerPlugin;

impl Plugin for NetworkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkManager>();

        app.add_systems(PostStartup, start_listening_system);

        app.add_systems(PreUpdate, handle_pending_connections_system);
    }
}
