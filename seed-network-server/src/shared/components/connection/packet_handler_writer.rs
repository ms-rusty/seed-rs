use bevy::prelude::Component;
use tokio::task::JoinHandle;

#[derive(Component)]
pub struct ConnectionPacketHandlerWriter {
    pub task: JoinHandle<()>,
}

impl ConnectionPacketHandlerWriter {
    pub fn new(task: JoinHandle<()>) -> Self {
        Self { task }
    }
}
