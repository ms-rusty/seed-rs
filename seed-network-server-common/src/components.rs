use bevy::prelude::Component;

#[derive(Component)]
pub struct ClientMessagePingRequest {
    pub payload: i64,
}
