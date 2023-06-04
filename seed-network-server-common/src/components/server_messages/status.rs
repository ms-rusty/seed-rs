use bevy::prelude::Component;
use serde::Serialize;

#[derive(Component, Debug, Serialize)]
pub struct ServerPingResponseMessage {
    pub payload: i64,
}

#[derive(Component, Debug, Serialize)]
pub struct ServerStatusResponseMessage {
    pub version: ServerVersion,
    pub players: ServerPlayers,
    pub description: ServerDescription,
    pub favicon: Option<String>,
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: bool,
    #[serde(rename = "previewsChat")]
    pub previews_chat: bool,
}

#[derive(Debug, Serialize)]
pub struct ServerVersion {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Serialize)]
pub struct ServerPlayers {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<ServerPlayersSample>,
}

#[derive(Debug, Serialize)]
pub struct ServerPlayersSample {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct ServerDescription {
    pub text: String,
}
