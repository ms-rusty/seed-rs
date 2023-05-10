use std::time::Duration;

use bevy::prelude::{App, Commands, Deref, EventWriter, Plugin, Res, ResMut, Resource};
use bevy_tokio_tasks::TokioTasksRuntime;

use crate::common::{ConnectionResult, NetworkConnectionEvent};

pub struct NetworkListenerPlugin;

impl Plugin for NetworkListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_listener_system);
        app.add_system(network_connection_receiver_system);
    }
}

fn create_listener_system(mut commands: Commands, tokio_runtime: ResMut<TokioTasksRuntime>) {
    let (tx, rx) = crossbeam_channel::unbounded::<ConnectionResult>();

    tokio_runtime.spawn_background_task(|mut task_context| async move {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:65535")
            .await
            .expect("Error on bind TcpListener.");

        loop {
            let connection = listener.accept().await;
            tx.send(connection).ok();
        }
    });

    commands.insert_resource(NetworkConnectionReceiver(rx));
}

#[derive(Resource, Deref)]
struct NetworkConnectionReceiver(crossbeam_channel::Receiver<ConnectionResult>);

fn network_connection_receiver_system(
    receiver: Res<NetworkConnectionReceiver>,
    mut events: EventWriter<NetworkConnectionEvent>,
) {
    for connection_result in receiver.try_iter() {
        events.send(NetworkConnectionEvent::from(connection_result));
    }
}
