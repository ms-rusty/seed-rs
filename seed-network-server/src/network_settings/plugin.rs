use bevy::{
    app::AppExit,
    prelude::{error, App, EventWriter, In, IntoSystem, IntoSystemConfigs, Plugin, PreStartup},
};

use super::{
    resources::NetworkSettings,
    systems::{create_settings_system, load_settings_system},
};

pub struct NetworkSettingsPlugin;

impl Plugin for NetworkSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkSettings>();

        app.add_systems(
            PreStartup,
            (
                create_settings_system.pipe(handle_error),
                load_settings_system.pipe(handle_error),
            )
                .chain(),
        );
    }
}

fn handle_error(
    In(result): In<Result<(), anyhow::Error>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if let Err(err) = result {
        error!("{:?}", err);

        app_exit_events.send(AppExit);
    }
}
