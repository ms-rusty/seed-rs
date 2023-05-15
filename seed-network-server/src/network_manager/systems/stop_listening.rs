use bevy::prelude::ResMut;

use crate::network_manager::resources::NetworkManager;

pub fn stop_listening_system(mut network_manager: ResMut<NetworkManager>) {
    if let Some(listener_handler) = network_manager.listener_handler.take() {
        listener_handler.abort();
    }
}
