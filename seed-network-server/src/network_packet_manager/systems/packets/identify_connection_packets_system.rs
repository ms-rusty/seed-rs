use bevy::prelude::*;
use num_traits::FromPrimitive;

use crate::{
    network_packet_manager::components::{
        ClientHandshakePacket, ClientHandshakingPackets, Packet, UndefinedPacket,
    },
    shared::{
        Connection, ConnectionHandshakingState, ConnectionPacketHandlerReader,
        ConnectionPacketHandlerWriter, ConnectionStatusState,
    },
};

pub fn handshaking_packet_system(
    mut commands: Commands,
    connection_query: Query<
        Entity,
        (
            With<Connection>,
            With<ConnectionHandshakingState>,
            With<ConnectionPacketHandlerReader>,
            With<ConnectionPacketHandlerWriter>,
            With<Children>, // Filtra apenas com packets.
        ),
    >,
    query: Query<(&Parent, Entity, &Packet), With<UndefinedPacket>>,
) {
    for (connection_entity, packet_entity, packet) in &query {
        let connection = connection_query.get(connection_entity.get());
        match FromPrimitive::from_i32(packet.id.value) {
            Some(ClientHandshakingPackets::Handshake) => {
                let result = ClientHandshakePacket::try_from(packet);
            }
            _ => {}
        }
    }
}

pub fn status_packet_system(
    commands: Commands,
    connection_query: Query<
        Entity,
        (
            With<Connection>,
            With<ConnectionStatusState>,
            With<Children>, // Filtra apenas com packets.
        ),
    >,
    query: Query<(&Parent, Entity, &Packet), With<UndefinedPacket>>,
) {
    for (connection_entity, packet_entity, packet) in &query {
        let connection = connection_query.get(connection_entity.get());
    }
}
