use bevy::prelude::{Commands, Entity, Query};
use seed_network_server_common::ServerPingResponseMessage;

use crate::network_packet_writer_manager::components::ServerPingResponsePacket;

pub fn ping_response_message_mapping_system(
    mut commands: Commands,
    query: Query<(Entity, &ServerPingResponseMessage)>,
) {
    for (message_entity, message) in &query {
        commands
            .entity(message_entity)
            .remove::<ServerPingResponseMessage>()
            .insert(ServerPingResponsePacket {
                payload: message.payload,
            });
    }
}
