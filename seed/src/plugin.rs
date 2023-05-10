use bevy::prelude::{App, Commands, Plugin};

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(load_configuration_system);
    }
}

fn load_configuration_system(mut commands: Commands) {
    commands.spawn_empty();
    println!("...");
}
