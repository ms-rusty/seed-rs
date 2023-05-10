use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    prelude::{
        default, App, FrameCountPlugin, TaskPoolOptions, TaskPoolPlugin, TypeRegistrationPlugin,
    },
    time::TimePlugin,
};
use bevy_tokio_tasks::TokioTasksPlugin;
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
    app.add_plugin(TokioTasksPlugin {
        make_runtime: Box::new(|| {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime for background tasks")
        }),
    });

    app.run();
}
