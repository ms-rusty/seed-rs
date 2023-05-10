use bevy::prelude::{App, Commands, Component, EventReader, Plugin, Resource};
use tokio::io::BufWriter;

use crate::common::{ConnectionSuccess, NetworkConnectionEvent};

pub struct NetworkConnectionPlugin;

impl Plugin for NetworkConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(connection_event_system);
    }
}

fn connection_event_system(
    mut commands: Commands,
    mut network_connection_events: EventReader<NetworkConnectionEvent>,
) {
    for network_connection_event in &mut network_connection_events {
        if let NetworkConnectionEvent::Success(connection) = network_connection_event {
            commands.insert_resource(Connection {
                stream: connection.0,
            });
        }
    }
}

#[derive(Resource)]
struct Connection {
    stream: tokio::net::TcpStream,
}
