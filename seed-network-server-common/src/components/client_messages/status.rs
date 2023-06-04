use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct ClientStatusRequestMessage;

#[derive(Component, Debug)]
pub struct ClientPingRequestMessage {
    pub payload: i64,
}
