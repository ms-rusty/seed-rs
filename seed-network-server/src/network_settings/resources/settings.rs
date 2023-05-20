use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize)]
pub struct NetworkSettings {
    pub server_address: SocketAddr,
    pub max_packet_length: u32,
    pub use_nagle_algorithm: bool,
    pub runtime_threads: usize,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            server_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565),
            max_packet_length: Default::default(),
            use_nagle_algorithm: false,
            runtime_threads: 1,
        }
    }
}
