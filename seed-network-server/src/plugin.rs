use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use crate::{network_manager::NetworkManagerPlugin, network_settings::NetworkSettingsPlugin};

pub struct NetworkServerPlugins;

impl PluginGroup for NetworkServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(NetworkSettingsPlugin)
            .add(NetworkManagerPlugin)
    }
}
