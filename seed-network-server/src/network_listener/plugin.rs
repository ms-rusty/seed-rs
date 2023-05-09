use bevy::prelude::{App, Plugin, ResMut};
use bevy_tokio_tasks::TokioTasksRuntime;

use crate::common::NetworkConnectionEvent;

pub struct NetworkListenerPlugin;

impl Plugin for NetworkListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_listener_system);
    }
}

fn create_listener_system(tokio_runtime: ResMut<TokioTasksRuntime>) {
    tokio_runtime.spawn_background_task(|mut task_context| async move {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:65535")
            .await
            .expect("Error on bind TcpListener.");

        loop {
            let connection = listener.accept().await;

            task_context
                .run_on_main_thread(move |bevy_context| {
                    bevy_context
                        .world
                        .send_event(NetworkConnectionEvent::from(connection));
                })
                .await;
        }
    });
}
