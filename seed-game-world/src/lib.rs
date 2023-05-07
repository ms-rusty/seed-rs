use bevy::prelude::App;
use bevy::{
    app::ScheduleRunnerPlugin,
    prelude::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin},
    time::TimePlugin,
};

pub struct GameWorld {
    app: App,
}

impl GameWorld {
    pub fn new() -> Self {
        // Create the Bevy App.
        let mut app = App::new();

        // Add minimal plugins.
        app.add_plugin(TaskPoolPlugin::default());
        app.add_plugin(TypeRegistrationPlugin::default());
        app.add_plugin(FrameCountPlugin::default());
        app.add_plugin(TimePlugin::default());
        app.add_plugin(ScheduleRunnerPlugin::default());

        // app.add_plugin(NetworkPlugin);

        Self { app }
    }

    pub fn run(mut self) {
        self.app.run();
    }
}
