use bevy::prelude::{Commands, Entity, Query, With};
use seed_network_server_common::ClientStatusRequestMessage;

use crate::{
    network_packet_reader_manager::components::{
        ClientStatusRequestPacket, ClientStatusRequestPacketId,
    },
    shared::PacketData,
};

pub fn status_request_message_mapping_system(
    mut commands: Commands,
    query: Query<(Entity, &PacketData), With<ClientStatusRequestPacketId>>,
) {
    for (packet_entity, packet_data) in &query {
        match ClientStatusRequestPacket::try_from(&packet_data.data) {
            Ok(_) => {
                commands
                    .entity(packet_entity)
                    .remove::<ClientStatusRequestPacketId>()
                    .insert(ClientStatusRequestMessage);
            }
            Err(_) => {}
        }
    }
}
