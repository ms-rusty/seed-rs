use bevy::{
    app::ScheduleRunnerPlugin,
    log::LogPlugin,
    prelude::{App, FrameCountPlugin, TaskPoolOptions, TaskPoolPlugin, TypeRegistrationPlugin},
    time::TimePlugin,
};
use bevy_tokio_runtime::TokioRuntimePlugin;

use seed_common::AppState;
use seed_database_server::DatabaseServerPlugin;
use seed_game_world::GameWorldPlugins;
use seed_network_server::NetworkServerPlugins;

use crate::{plugin::SeedPlugin, resources::SeedSettings};

pub fn run_app(settings: SeedSettings) {
    let mut app = App::new();

    // Add bevy plugins.
    app.add_plugin(TaskPoolPlugin {
        task_pool_options: TaskPoolOptions::with_num_threads(settings.task_pool_threads),
    });
    app.add_plugin(TypeRegistrationPlugin::default());
    app.add_plugin(FrameCountPlugin::default());
    app.add_plugin(TimePlugin::default());
    app.add_plugin(ScheduleRunnerPlugin::run_loop(
        std::time::Duration::from_secs_f64(1.0 / settings.tick_rate_hz),
    ));
    app.add_plugin(LogPlugin::default());

    // Add seed plugins.
    app.add_plugin(SeedPlugin);
    app.add_plugin(DatabaseServerPlugin);
    app.add_plugins(GameWorldPlugins);
    app.add_plugins(NetworkServerPlugins);

    // Add tokio plugin.
    app.add_plugin(TokioRuntimePlugin::default());

    app.add_state::<AppState>();

    app.insert_resource(settings);

    app.run();
}
