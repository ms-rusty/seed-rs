use bevy::prelude::{Component, Resource};
use tokio::{net::TcpListener, task::JoinHandle};

#[derive(Component)]
pub struct Listener(pub TcpListener);

#[derive(Resource)]
pub struct ListenerTask {
    pub task: JoinHandle<()>,
}
