use bevy::prelude::Component;
use std::sync::Arc;
use tokio::{io::BufReader, net::tcp::OwnedReadHalf, sync::Mutex};

#[derive(Component)]
pub struct ConnectionStreamReader {
    pub reader: Arc<Mutex<BufReader<OwnedReadHalf>>>,
}

impl ConnectionStreamReader {
    pub fn new(read: OwnedReadHalf) -> Self {
        Self {
            reader: Arc::new(Mutex::new(BufReader::new(read))),
        }
    }
}
