use bevy::prelude::Component;

#[derive(Component)]
pub struct ClientStatusRequestMessage;

#[derive(Component)]
pub struct ClientPingRequestMessage {
    pub payload: i64,
}
