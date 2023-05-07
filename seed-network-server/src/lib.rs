use bevy::prelude::App;
use bevy::{
    app::ScheduleRunnerPlugin,
    prelude::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin},
    time::TimePlugin,
};
use plugin::NetworkServerPlugin;

mod network_channel;
mod plugin;
mod resources;

pub struct NetworkServer {
    app: App,
}

impl NetworkServer {
    pub fn new() -> Self {
        let mut app = App::new();

        // Add minimal plugins.
        app.add_plugin(TaskPoolPlugin::default());
        app.add_plugin(TypeRegistrationPlugin::default());
        app.add_plugin(FrameCountPlugin::default());
        app.add_plugin(TimePlugin::default());
        app.add_plugin(ScheduleRunnerPlugin::default());

        app.add_plugin(NetworkServerPlugin);

        Self { app }
    }

    pub fn run(mut self) {
        self.app.run();
    }
}
