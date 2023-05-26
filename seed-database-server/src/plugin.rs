use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

pub struct DatabaseServerPlugins;

impl PluginGroup for DatabaseServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}
