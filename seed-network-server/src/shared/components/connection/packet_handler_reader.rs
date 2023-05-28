use bevy::prelude::Component;
use tokio::task::JoinHandle;

#[derive(Component)]
pub struct ConnectionPacketHandlerReader {
    pub task: JoinHandle<()>,
}

impl ConnectionPacketHandlerReader {
    pub fn new(task: JoinHandle<()>) -> Self {
        Self { task }
    }
}
