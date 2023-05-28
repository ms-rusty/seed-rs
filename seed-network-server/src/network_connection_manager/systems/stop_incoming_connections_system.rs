use bevy::prelude::ResMut;

use crate::shared::NetworkListener;

pub fn stop_incoming_connections_system(mut network_listener: ResMut<NetworkListener>) {
    if let Some(listener_handler) = network_listener.accept_connection_task.take() {
        listener_handler.abort();
    }
}
