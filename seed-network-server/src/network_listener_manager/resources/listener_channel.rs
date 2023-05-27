use bevy::prelude::Resource;
use crossbeam_channel::{Receiver, Sender};
use tokio::net::TcpListener;

type ListenerResult = Result<TcpListener, std::io::Error>;

#[derive(Resource)]
pub struct ListenerChannel {
    pub sender: Sender<ListenerResult>,
    pub receiver: Receiver<ListenerResult>,
}

impl Default for ListenerChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded::<ListenerResult>(1);
        Self { sender, receiver }
    }
}
