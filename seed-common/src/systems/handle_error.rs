use bevy::{
    app::AppExit,
    prelude::{error, EventWriter, In},
};

pub fn handle_error(
    In(result): In<Result<(), anyhow::Error>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if let Err(err) = result {
        error!("{:?}", err);

        app_exit_events.send(AppExit);
    }
}
