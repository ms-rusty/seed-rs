use bevy::prelude::{Children, Commands, Entity, Parent, Query, With};

use crate::shared::{Connection, ConnectionPlayState, PacketId};

pub fn play_packets_mapping_system(
    mut commands: Commands,
    connection_query: Query<
        Entity,
        (
            With<Connection>,
            With<ConnectionPlayState>,
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

        // match FromPrimitive::from_i32(packet_id.value) {
        //     ClientPlayPackets
        // };
    }
}
