use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    prelude::{
        App, FrameCountPlugin, States, TaskPoolOptions, TaskPoolPlugin, TypeRegistrationPlugin,
    },
    time::TimePlugin,
};
use bevy_tokio_runtime::TokioRuntimePlugin;
use plugin::SeedPlugin;
use seed_database_server::DatabaseServerPlugin;
use seed_game_world::GameWorldPlugin;
use seed_network_server::NetworkServerPlugin;

mod plugin;

fn main() {
    let mut app = App::new();

    // App Scheduler
    app.insert_resource(ScheduleRunnerSettings::run_loop(
        std::time::Duration::from_secs_f64(1.0 / 2.0), // 20 fps.
    ));

    // Add bevy minimal plugins.
    app.add_plugin(TaskPoolPlugin {
        task_pool_options: TaskPoolOptions::with_num_threads(2),
    });
    app.add_plugin(TypeRegistrationPlugin::default());
    app.add_plugin(FrameCountPlugin::default());
    app.add_plugin(TimePlugin::default());
    app.add_plugin(ScheduleRunnerPlugin::default());

    // Add seed plugins.
    app.add_plugin(SeedPlugin);
    app.add_plugin(DatabaseServerPlugin);
    app.add_plugin(GameWorldPlugin);
    app.add_plugin(NetworkServerPlugin);

    // Tokio plugin.
    app.add_plugin(TokioRuntimePlugin::default());

    app.run();
}

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum AppState {
    #[default]
    Loading,
    StartingServices,
    Running,
}
