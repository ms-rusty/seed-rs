use bevy::prelude::*;
use num_traits::FromPrimitive;

use crate::{
    network_packet_manager::components::{
        ClientEncryptionResponsePacketId, ClientHandshakePacketId, ClientHandshakingPackets,
        ClientLoginPackets, ClientLoginPluginResponsePacketId, ClientLoginStartPacketId,
        ClientPingRequestPacketId, ClientStatusPackets, ClientStatusRequestPacketId, PacketId,
        UndefinedPacket,
    },
    shared::{
        Connection, ConnectionHandshakingState, ConnectionLoginState,
        ConnectionPacketHandlerReader, ConnectionPacketHandlerWriter, ConnectionStatusState,
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
            With<Children>, // Only with packets.
        ),
    >,
    query: Query<(&Parent, Entity, &PacketId)>,
) {
    for (connection_entity, packet_entity, packet_id) in &query {
        if let Err(err) = connection_query.get(connection_entity.get()) {
            continue;
        }

        commands.entity(packet_entity).remove::<PacketId>();

        match FromPrimitive::from_i32(packet_id.value) {
            Some(ClientHandshakingPackets::Handshake) => {
                commands
                    .entity(packet_entity)
                    .insert(ClientHandshakePacketId);
            }
            _ => {
                commands
                    .entity(packet_entity)
                    .insert(UndefinedPacket::new(packet_id.id));
            }
        }
    }
}

pub fn status_packet_system(
    mut commands: Commands,
    connection_query: Query<
        Entity,
        (
            With<Connection>,
            With<ConnectionStatusState>,
            With<Children>, // Only with packets.
        ),
    >,
    query: Query<(&Parent, Entity, &PacketId)>,
) {
    for (connection_entity, packet_entity, packet_id) in &query {
        if let Err(err) = connection_query.get(connection_entity.get()) {
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

pub fn login_packet_system(
    mut commands: Commands,
    connection_query: Query<
        Entity,
        (
            With<Connection>,
            With<ConnectionLoginState>,
            With<Children>, // Only with packets.
        ),
    >,
    query: Query<(&Parent, Entity, &PacketId)>,
) {
    for (connection_entity, packet_entity, packet_id) in &query {
        if let Err(err) = connection_query.get(connection_entity.get()) {
            continue;
        }

        commands.entity(packet_entity).remove::<PacketId>();

        match FromPrimitive::from_i32(packet_id.value) {
            Some(ClientLoginPackets::LoginStart) => {
                commands
                    .entity(packet_entity)
                    .insert(ClientLoginStartPacketId);
            }
            Some(ClientLoginPackets::EncryptionResponse) => {
                commands
                    .entity(packet_entity)
                    .insert(ClientEncryptionResponsePacketId);
            }
            Some(ClientLoginPackets::LoginPluginResponse) => {
                commands
                    .entity(packet_entity)
                    .insert(ClientLoginPluginResponsePacketId);
            }
            _ => {
                commands
                    .entity(packet_entity)
                    .insert(UndefinedPacket::new(packet_id.id));
            }
        };
    }
}
