use bevy::prelude::Component;
use uuid::Uuid;

#[derive(Component, Debug)]
pub struct ClientLoginStartMessage {
    pub username: String,
    pub has_player_uuid: bool,
    pub player_uuid: Option<Uuid>,
}
