use arrayvec::ArrayVec;
use bevy::prelude::{Commands, Entity, Query, With};
use seed_network_server_common::ClientLoginPluginResponseMessage;

use crate::{
    network_packet_reader_manager::components::{
        ClientLoginPluginResponsePacket, ClientLoginPluginResponsePacketId,
        ClientLoginStartPacketId,
    },
    shared::PacketData,
};

pub fn login_plugin_response_message_mapping_system(
    mut commands: Commands,
    query: Query<(Entity, &PacketData), With<ClientLoginPluginResponsePacketId>>,
) {
    for (packet_entity, packet_data) in &query {
        match ClientLoginPluginResponsePacket::try_from(&packet_data.data) {
            Ok(packet) => {
                let data = packet.data.map(|bytes| {
                    let mut data = ArrayVec::new();
                    data.extend(bytes.to_vec());
                    data
                });

                commands
                    .entity(packet_entity)
                    .remove::<ClientLoginStartPacketId>()
                    .insert(ClientLoginPluginResponseMessage {
                        message_id: packet.message_id.value,
                        successful: packet.successful,
                        data,
                    });
            }
            Err(_) => {}
        }
    }
}
