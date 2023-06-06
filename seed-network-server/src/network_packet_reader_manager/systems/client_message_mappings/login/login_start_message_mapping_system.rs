use bevy::prelude::{Commands, Entity, Query, With};
use seed_network_server_common::ClientLoginStartMessage;

use crate::{
    network_packet_reader_manager::components::{ClientLoginStartPacket, ClientLoginStartPacketId},
    shared::PacketData,
};

pub fn login_start_message_mapping_system(
    mut commands: Commands,
    query: Query<(Entity, &PacketData), With<ClientLoginStartPacketId>>,
) {
    for (packet_entity, packet_data) in &query {
        match ClientLoginStartPacket::try_from(&packet_data.data) {
            Ok(packet) => {
                commands
                    .entity(packet_entity)
                    .remove::<ClientLoginStartPacketId>()
                    .insert(ClientLoginStartMessage {
                        username: packet.username.to_owned(),
                        has_player_uuid: packet.has_player_uuid,
                        player_uuid: packet.player_uuid,
                    });
            }
            Err(_) => {}
        }
    }
}
