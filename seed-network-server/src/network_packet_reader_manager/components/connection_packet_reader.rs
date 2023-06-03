use bevy::prelude::Component;
use tokio::task::JoinHandle;

#[derive(Component)]
pub struct ConnectionPacketReader {
    pub task: JoinHandle<()>,
}

impl ConnectionPacketReader {
    pub fn new(task: JoinHandle<()>) -> Self {
        Self { task }
    }
}
