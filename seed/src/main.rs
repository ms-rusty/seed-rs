use bevy::{
    app::ScheduleRunnerPlugin,
    log::LogPlugin,
    prelude::{App, FrameCountPlugin, TaskPoolOptions, TaskPoolPlugin, TypeRegistrationPlugin},
    time::TimePlugin,
};
use bevy_tokio_runtime::TokioRuntimePlugin;
use plugin::SeedPlugin;
use seed_common::AppState;
use seed_database_server::DatabaseServerPlugin;
use seed_game_world::GameWorldPlugins;
use seed_network_server::NetworkServerPlugins;

mod plugin;

fn main() {
    let mut app = App::new();

    app.add_state::<AppState>();

    // Add bevy minimal plugins.
    app.add_plugin(TaskPoolPlugin {
        task_pool_options: TaskPoolOptions::with_num_threads(2),
    });
    app.add_plugin(TypeRegistrationPlugin::default());
    app.add_plugin(FrameCountPlugin::default());
    app.add_plugin(TimePlugin::default());
    app.add_plugin(ScheduleRunnerPlugin {
        run_mode: bevy::app::RunMode::Loop {
            wait: Some(std::time::Duration::from_secs_f64(1.0 / 1.0)),
        },
    });
    app.add_plugin(LogPlugin::default());

    // Add seed plugins.
    app.add_plugin(SeedPlugin);
    app.add_plugin(DatabaseServerPlugin);
    app.add_plugins(GameWorldPlugins);
    app.add_plugins(NetworkServerPlugins);

    // Tokio plugin.
    app.add_plugin(TokioRuntimePlugin::default());

    app.run();
}
