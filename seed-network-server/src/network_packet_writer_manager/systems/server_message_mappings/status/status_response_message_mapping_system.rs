use bevy::prelude::{error, Commands, Entity, Query};
use seed_network_server_common::ServerStatusResponseMessage;

use crate::network_packet_writer_manager::components::ServerStatusResponsePacket;

pub fn status_response_message_mapping_system(
    mut commands: Commands,
    query: Query<(Entity, &ServerStatusResponseMessage)>,
) {
    for (message_entity, message) in &query {
        let Ok(response) = serde_json::to_string(&message) else {
            error!("Error on serialize message.");
            continue;
        };

        commands
            .entity(message_entity)
            .remove::<ServerStatusResponseMessage>()
            .insert(ServerStatusResponsePacket {
                json_response: response,
            });
    }
}
