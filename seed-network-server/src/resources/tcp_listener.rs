use std::sync::Arc;

use bevy::prelude::{Deref, Resource};

use crate::network_channel::NetworkEventChannel;

#[derive(Resource, Debug)]
pub struct NetworkTcpListener(pub Arc<tokio::net::TcpListener>);

#[derive(Resource, Debug, Deref)]
pub struct TcpChannel {
    pub receiver: crossbeam_channel::Receiver<NetworkTcpListener>,
}

#[derive(Resource, Deref)]
pub struct AcceptChannel {
    pub receiver: crossbeam_channel::Receiver<NetworkEventChannel>,
}
