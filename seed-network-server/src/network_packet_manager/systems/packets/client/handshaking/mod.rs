use bevy::prelude::{BuildChildren, Commands, Entity, Parent, Query, With};

use crate::{
    network_packet_manager::components::{
        ClientHandshakePacket, ClientHandshakePacketId, NextState, PacketData,
    },
    shared::{ConnectionHandshakingState, ConnectionLoginState, ConnectionStatusState},
};

pub fn handshake_system(
    mut commands: Commands,
    query: Query<(&Parent, Entity, &PacketData), With<ClientHandshakePacketId>>,
) {
    for (connection_entity, packet_entity, packet_data) in &query {
        commands
            .entity(connection_entity.get())
            .remove::<ConnectionHandshakingState>();

        match ClientHandshakePacket::try_from(&packet_data.data) {
            Ok(packet) => match packet.next_state {
                NextState::Status(next_packet) => {
                    commands
                        .entity(connection_entity.get())
                        .insert(ConnectionStatusState);

                    if let Some(next_packet) = next_packet {
                        commands
                            .entity(connection_entity.get())
                            .with_children(|parent| {
                                parent.spawn(next_packet);
                            });
                    }
                }
                NextState::Login(next_packet) => {
                    commands
                        .entity(connection_entity.get())
                        .insert(ConnectionLoginState);

                    commands
                        .entity(connection_entity.get())
                        .with_children(|parent| {
                            parent.spawn(next_packet);
                        });
                }
            },
            Err(_) => {}
        }

        commands.entity(packet_entity).despawn();
    }
}
