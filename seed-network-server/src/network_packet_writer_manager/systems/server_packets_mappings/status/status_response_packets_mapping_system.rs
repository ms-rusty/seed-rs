use bevy::prelude::{Commands, Entity, Parent, Query};

use crate::{
    network_packet_writer_manager::components::ServerStatusResponsePacket,
    shared::ConnectionServerPacketsChannel,
};

pub fn status_response_packets_mapping_system(
    mut commands: Commands,
    packets_channel_query: Query<&ConnectionServerPacketsChannel>,
    query: Query<(&Parent, Entity, &ServerStatusResponsePacket)>,
) {
    for (connection_entity, packet_entity, status_response_packet) in &query {
        match packets_channel_query.get(connection_entity.get()) {
            Ok(packets_channel) => {
                if let Err(_) = packets_channel.sender.send(status_response_packet.into()) {
                    // Error on send packet.
                }
            }
            Err(_) => {}
        }

        commands.entity(packet_entity).despawn();
    }
}
