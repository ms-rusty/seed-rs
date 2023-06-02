use bevy::prelude::{info, Commands, Component, Entity, EventWriter, Query, With};
use seed_network_server_common::{ClientMessage, ClientMessagePingRequest};

use crate::network_packet_manager::components::{
    ClientPingRequestPacket, ClientPingRequestPacketId, ClientStatusRequestPacket,
    ClientStatusRequestPacketId, PacketData,
};

pub fn status_request_system(
    mut commands: Commands,
    query: Query<(Entity, &PacketData), With<ClientStatusRequestPacketId>>,
    mut event_writer: EventWriter<ClientMessage>,
) {
    for (packet_entity, packet_data) in &query {
        match ClientStatusRequestPacket::try_from(&packet_data.data) {
            Ok(packet) => {
                event_writer.send(ClientMessage::StatusRequest);
            }
            Err(_) => {}
        }

        commands.entity(packet_entity).despawn();
    }
}

pub fn ping_request_system(
    mut commands: Commands,
    query: Query<(Entity, &PacketData), With<ClientPingRequestPacketId>>,
    mut event_writer: EventWriter<ClientMessage>,
) {
    for (packet_entity, packet_data) in &query {
        match ClientPingRequestPacket::try_from(&packet_data.data) {
            Ok(packet) => {
                // Entity, PacketData, ClientPingRequestPacketId

                // Entity

                // Entity, ClientMessagePingRequest

                commands
                    .entity(packet_entity)
                    .insert(ClientMessagePingRequest {
                        payload: packet.payload,
                    });

                event_writer.send(ClientMessage::PingRequest {
                    payload: packet.payload,
                });
            }
            Err(_) => {}
        }

        commands.entity(packet_entity).despawn();
    }
}

#[derive(Component)]
pub enum A {
    B {},
}
