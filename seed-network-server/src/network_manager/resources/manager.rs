use std::collections::HashMap;

use bevy::prelude::Resource;
use tokio::task::JoinHandle;

use crate::network_manager::events::{Client, ClientId};

#[derive(Resource, Default)]
pub struct NetworkManager {
    pub listener_handler: Option<JoinHandle<()>>,
}

impl NetworkManager {
    // pub fn disconnect(&self, connection: Connection) -> Result<()> {
    //     let connection = if let Some(connection) = self.established_connections.remove(&connection)
    //     {
    //         connection
    //     } else {
    //         return Err(""NetworkError::ConnectionNotFound(conn_id)"");
    //     };

    //     connection.1.stop();
    // }
}

// #[derive(Resource)]
// struct Listening(pub crossbeam_channel::Receiver<()>);
