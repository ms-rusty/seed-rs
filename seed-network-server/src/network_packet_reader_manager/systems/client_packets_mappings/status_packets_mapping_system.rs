use bevy::prelude::{Children, Commands, Entity, Parent, Query, With};
use num_traits::FromPrimitive;

use crate::{
    network_packet_reader_manager::components::{
        ClientPingRequestPacketId, ClientStatusPackets, ClientStatusRequestPacketId,
        UndefinedPacket,
    },
    shared::{Connection, ConnectionStatusState, PacketId},
};

pub fn status_packets_mapping_system(
    mut commands: Commands,
    connection_query: Query<
        Entity,
        (
            With<Connection>,
            With<ConnectionStatusState>,
            With<Children>, // Only connections with children (packets, messages...).
        ),
    >,
    query: Query<(&Parent, Entity, &PacketId)>,
) {
    for (connection_entity, packet_entity, packet_id) in &query {
        if let Err(_) = connection_query.get(connection_entity.get()) {
            continue;
        }

        commands.entity(packet_entity).remove::<PacketId>();

        match FromPrimitive::from_i32(packet_id.value) {
            Some(ClientStatusPackets::StatusRequest) => {
                commands
                    .entity(packet_entity)
                    .insert(ClientStatusRequestPacketId);
            }
            Some(ClientStatusPackets::PingRequest) => {
                commands
                    .entity(packet_entity)
                    .insert(ClientPingRequestPacketId);
            }
            _ => {
                commands
                    .entity(packet_entity)
                    .insert(UndefinedPacket::new(packet_id.id));
            }
        };
    }
}
