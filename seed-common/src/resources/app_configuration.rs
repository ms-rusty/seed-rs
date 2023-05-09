use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Resource, Default, Deserialize)]
pub struct AppConfiguration {
    tokio_runtime_threads: u8,
}
