use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct NetworkSettings {
    pub nagle_algorithm: bool,
    pub max_packet_length: u32,
}
