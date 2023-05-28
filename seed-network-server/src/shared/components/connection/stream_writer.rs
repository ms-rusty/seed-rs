use std::sync::Arc;

use bevy::prelude::Component;
use tokio::{io::BufWriter, net::tcp::OwnedWriteHalf, sync::Mutex};

#[derive(Component)]
pub struct ConnectionStreamWriter {
    pub writer: Arc<Mutex<BufWriter<OwnedWriteHalf>>>,
}

impl ConnectionStreamWriter {
    pub fn new(write: OwnedWriteHalf) -> Self {
        Self {
            writer: Arc::new(Mutex::new(BufWriter::new(write))),
        }
    }
}
