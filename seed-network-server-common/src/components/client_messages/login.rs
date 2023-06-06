use arrayvec::ArrayVec;
use bevy::prelude::Component;
use uuid::Uuid;

#[derive(Component, Debug)]
pub struct ClientLoginStartMessage {
    pub username: String,
    pub has_player_uuid: bool,
    pub player_uuid: Option<Uuid>,
}

const MAX_DATA_LENGTH: usize = 1048576;

#[derive(Component, Debug)]
pub struct ClientLoginPluginResponseMessage {
    pub message_id: i32,
    pub successful: bool,
    pub data: Option<ArrayVec<u8, MAX_DATA_LENGTH>>, // byte array
}
