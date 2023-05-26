use crate::{network_manager::NetworkChannel, shared::events::ConnectionRequest};

pub struct ConnectionRequestChannel {
    pub channel: NetworkChannel<ConnectionRequest>,
}
