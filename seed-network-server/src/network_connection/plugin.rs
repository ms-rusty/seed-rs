use bevy::prelude::{App, EventReader, Plugin};

use crate::common::NetworkConnectionEvent;

pub struct NetworkConnectionPlugin;

impl Plugin for NetworkConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(connection_event_system);
    }
}

fn connection_event_system(mut network_connection_events: EventReader<NetworkConnectionEvent>) {
    for connection_event in &mut network_connection_events {
        let NetworkConnectionEvent::Success(connection) = connection_event else {
            println!("Connection error: {:?}", connection_event);
            continue;
        };

        println!("New connection: {:?}", connection);
    }
}
