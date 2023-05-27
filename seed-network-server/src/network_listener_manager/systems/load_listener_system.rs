use bevy::prelude::{Commands, Res};

use crate::{network_listener_manager::resources::ListenerChannel, shared::NetworkListener};

pub fn load_listener_system(
    mut commands: Commands,
    listener_channel: Res<ListenerChannel>,
) -> Result<(), anyhow::Error> {
    if let Ok(listener) = listener_channel.receiver.try_recv() {
        let listener = listener?;
        commands.insert_resource(NetworkListener::new(listener));
    }

    Ok(())
}
