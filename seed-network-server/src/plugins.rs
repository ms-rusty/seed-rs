use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use crate::{
    network_connection_manager::NetworkConnectionManagerPlugin,
    network_listener_manager::NetworkListenerManagerPlugin,
    network_message_manager::NetworkMessageManagerPlugin,
    network_packet_reader_manager::NetworkPacketReaderManagerPlugin,
    network_runtime::NetworkRuntimePlugin, network_settings::NetworkSettingsPlugin,
};

pub struct NetworkServerPlugins;

impl PluginGroup for NetworkServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(NetworkSettingsPlugin)
            .add(NetworkRuntimePlugin)
            .add(NetworkListenerManagerPlugin)
            .add(NetworkConnectionManagerPlugin)
            .add(NetworkMessageManagerPlugin)
            .add(NetworkPacketReaderManagerPlugin)
    }
}
