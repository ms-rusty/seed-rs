use bevy::prelude::Component;
use tokio::task::JoinHandle;

#[derive(Component)]
pub struct ConnectionPacketWriter {
    pub task: JoinHandle<()>,
}

impl ConnectionPacketWriter {
    pub fn new(task: JoinHandle<()>) -> Self {
        Self { task }
    }
}
