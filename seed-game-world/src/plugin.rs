use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use crate::{game_client_message::GameClientMessagePlugin, game_settings::GameSettingsPlugin};

pub struct GameWorldPlugins;

impl PluginGroup for GameWorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GameSettingsPlugin)
            .add(GameClientMessagePlugin)
    }
}
