use bevy::prelude::{info, App, EventReader, EventWriter, Plugin, Update};
use seed_network_server_common::{ClientMessage, ServerMessage};

pub struct GameClientMessagePlugin;

impl Plugin for GameClientMessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClientMessage>();
        app.add_event::<ServerMessage>();
        app.add_systems(Update, system);
    }
}

fn system(
    mut event_reader: EventReader<ClientMessage>,
    mut event_writer: EventWriter<ServerMessage<'static>>,
) {
    for event in event_reader.iter() {
        info!("ClientMessage");
        match event {
            ClientMessage::StatusRequest => {
                event_writer.send(ServerMessage::StatusResponse {
                    response: r#"{
                    "version": {
                        "name": "1.19.4",
                        "protocol": 762
                    },
                    "players": {
                        "max": 100,
                        "online": 5,
                        "sample": [
                            {
                                "name": "thinkofdeath",
                                "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
                            }
                        ]
                    },
                    "description": {
                        "text": "Hello world"
                    },
                    "favicon": "data:image/png;base64,<data>",
                    "enforcesSecureChat": true
                }"#,
                });
            }
            ClientMessage::PingRequest { payload } => {
                event_writer.send(ServerMessage::PingResponse { payload: *payload });
            }
        }
    }
}
