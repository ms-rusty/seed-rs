use bevy::prelude::{Commands, Entity, Query, With};
use seed_network_server_common::ClientPingRequestMessage;

use crate::{
    network_packet_reader_manager::components::{
        ClientPingRequestPacket, ClientPingRequestPacketId,
    },
    shared::PacketData,
};

pub fn ping_request_message_mapping_system(
    mut commands: Commands,
    query: Query<(Entity, &PacketData), With<ClientPingRequestPacketId>>,
) {
    for (packet_entity, packet_data) in &query {
        match ClientPingRequestPacket::try_from(&packet_data.data) {
            Ok(packet) => {
                // Entity, PacketData, ClientPingRequestPacketId

                commands
                    .entity(packet_entity)
                    .remove::<ClientPingRequestPacketId>();

                // Entity, PacketData

                // Entity, PacketData, ClientPingRequestMessage

                commands
                    .entity(packet_entity)
                    .insert(ClientPingRequestMessage {
                        payload: packet.payload,
                    });
            }
            Err(_) => {}
        }
    }
}
