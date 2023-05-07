use std::sync::Arc;

use bevy::prelude::*;
use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};
use tokio::net::TcpListener;

use crate::{
    network_channel::{ConnectionEvent, NetworkEventChannel},
    resources::{
        tcp_listener::{AcceptChannel, TcpChannel},
        NetworkTcpListener,
    },
};

pub struct NetworkServerPlugin2;

impl Plugin for NetworkServerPlugin2 {
    fn build(&self, app: &mut App) {
        app.add_plugin(TokioTasksPlugin::default());

        app.add_startup_system(startup_listener_system);

        app.add_system(
            create_listener_system.run_if(resource_exists::<TcpChannel>().and_then(
                |channel: Res<TcpChannel>| {
                    println!("{:?}", channel.receiver.len());
                    channel.receiver.len() > 0
                },
            )),
        );

        app.add_system(accept_listener_system.run_if(resource_removed::<TcpChannel>()));

        app.add_system(connection_event_system.run_if(resource_exists::<AcceptChannel>()));
    }
}

fn connection_event_system(channel: ResMut<AcceptChannel>) {
    while let Ok(network_event) = channel.try_recv() {
        match network_event {
            NetworkEventChannel::ConnectionEvent(connection) => match connection {
                ConnectionEvent::Success((stream, socket_addr)) => {
                    println!("stream: {:?}", stream);
                    println!("socket_addr: {:?}", socket_addr);
                }
                ConnectionEvent::Failure(error) => {
                    println!("error: {:?}", error);
                }
            },
            NetworkEventChannel::DisconnectionEvent(_) => println!("Bye bye!"),
        }
    }
}

fn startup_listener_system(mut commands: Commands, runtime: ResMut<TokioTasksRuntime>) {
    let (sender, receiver) = crossbeam_channel::bounded::<NetworkTcpListener>(1);

    runtime.spawn_background_task(|mut _context| async move {
        println!("startup_listener_system");

        let listener = TcpListener::bind("127.0.0.1:65535")
            .await
            .expect("Error on bind TcpListener.");

        println!("startup_listener_system pos");

        sender.send(NetworkTcpListener(Arc::from(listener))).ok();
    });

    commands.insert_resource(TcpChannel { receiver });

    println!("startup_listener_system end");
}

fn create_listener_system(mut commands: Commands, resource: Res<TcpChannel>) {
    println!("create_listener_system");
    while let Ok(listener) = resource.receiver.try_recv() {
        println!("create_listener_system try");
        commands.insert_resource(listener);
        commands.remove_resource::<TcpChannel>()
    }
}

fn accept_listener_system(
    mut commands: Commands,
    runtime: ResMut<TokioTasksRuntime>,
    listener: Res<NetworkTcpListener>,
) {
    let listener = listener.0.clone();
    let (sender, receiver) = crossbeam_channel::unbounded::<NetworkEventChannel>();

    runtime.spawn_background_task(|mut _context| async move {
        println!("accept_listener_system");

        loop {
            let accept = listener.accept().await;

            println!("accept_listener_system accept");

            sender
                .send(NetworkEventChannel::ConnectionEvent(ConnectionEvent::from(
                    accept,
                )))
                .ok();
        }
    });

    commands.insert_resource(AcceptChannel { receiver });
}
