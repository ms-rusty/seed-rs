use std::sync::Arc;

use bevy::prelude::Resource;
use tokio::{net::TcpListener, sync::Mutex, task::JoinHandle};

#[derive(Resource)]
pub struct NetworkListener {
    pub server_socket: Arc<Mutex<TcpListener>>,
    pub accept_connection_task: Option<JoinHandle<()>>,
}

impl NetworkListener {
    pub fn new(listener: TcpListener) -> Self {
        Self {
            server_socket: Arc::new(Mutex::new(listener)),
            accept_connection_task: None,
        }
    }
}
