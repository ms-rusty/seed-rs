use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use crate::{
    network_listener_manager::NetworkListenerManagerPlugin, network_manager::NetworkManagerPlugin,
    network_message_manager::NetworkMessageManagerPlugin,
    network_packet_manager::NetworkPacketManagerPlugin, network_settings::NetworkSettingsPlugin,
};

pub struct NetworkServerPlugins;

impl PluginGroup for NetworkServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(NetworkSettingsPlugin)
            .add(NetworkListenerManagerPlugin)
            .add(NetworkMessageManagerPlugin)
            .add(NetworkPacketManagerPlugin)
            .add(NetworkManagerPlugin)
    }
}
