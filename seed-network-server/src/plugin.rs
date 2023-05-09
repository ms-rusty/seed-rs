use bevy::prelude::{App, Plugin};

use crate::{
    common::NetworkConnectionEvent, network_connection::NetworkConnectionPlugin,
    network_listener::NetworkListenerPlugin,
};

pub struct NetworkServerPlugin;

impl Plugin for NetworkServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NetworkConnectionPlugin);
        app.add_plugin(NetworkListenerPlugin);

        app.add_event::<NetworkConnectionEvent>();
    }
}
