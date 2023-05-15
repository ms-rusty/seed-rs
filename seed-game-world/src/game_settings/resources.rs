use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct GameSettings {}
