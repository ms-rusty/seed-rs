use bevy::prelude::{App, Plugin};

mod network_channel;
mod plugin;
mod resources;

pub struct NetworkServerPlugin;

impl Plugin for NetworkServerPlugin {
    fn build(&self, app: &mut App) {}
}
