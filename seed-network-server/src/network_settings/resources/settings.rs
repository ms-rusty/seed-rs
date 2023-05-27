use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

pub const FILE_NAME: &str = "network_server_settings.toml";

#[derive(Resource, Serialize, Deserialize)]
pub struct NetworkSettings {
    pub server_address: SocketAddr,
    pub max_packet_length: u32,
    pub use_nagle_algorithm: bool,
    pub task_pool_threads: usize,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            server_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565),
            max_packet_length: Default::default(),
            use_nagle_algorithm: false,
            task_pool_threads: 1,
        }
    }
}
