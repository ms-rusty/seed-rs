use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Resource, Default, Deserialize)]
pub struct AppConfiguration {
    _tokio_runtime_threads: u8,
}
