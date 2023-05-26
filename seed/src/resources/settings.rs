use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

pub const SEED_SETTINGS_PATH: &str = "seed_settings.toml";

#[derive(Resource, Serialize, Deserialize)]
pub struct SeedSettings {
    pub task_pool_threads: usize,
    pub tick_rate_hz: f64,
}

impl Default for SeedSettings {
    fn default() -> Self {
        Self {
            task_pool_threads: 1,
            tick_rate_hz: 20.0,
        }
    }
}
