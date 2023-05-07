use bevy::{
    app::ScheduleRunnerPlugin,
    prelude::{App, FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin},
    time::TimePlugin,
};
use plugin::SeedPlugin;
use seed_database_server::DatabaseServerPlugin;
use seed_game_world::GameWorldPlugin;
use seed_network_server::NetworkServerPlugin;

mod plugin;

fn main() {
    let mut app = App::new();

    // Add bevy minimal plugins.
    app.add_plugin(TaskPoolPlugin::default());
    app.add_plugin(TypeRegistrationPlugin::default());
    app.add_plugin(FrameCountPlugin::default());
    app.add_plugin(TimePlugin::default());
    app.add_plugin(ScheduleRunnerPlugin::default());

    // Add seed plugins.
    app.add_plugin(SeedPlugin);
    app.add_plugin(DatabaseServerPlugin);
    app.add_plugin(GameWorldPlugin);
    app.add_plugin(NetworkServerPlugin);

    app.run();
}
