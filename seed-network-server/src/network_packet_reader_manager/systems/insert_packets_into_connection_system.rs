use bevy::prelude::{BuildChildren, Commands, Entity, Query, With};

use crate::{
    network_packet_reader_manager::components::ConnectionPacketReader,
    shared::{Connection, ConnectionClientPacketsChannel},
};

pub fn insert_packets_into_connection_system(
    mut commands: Commands,
    query: Query<
        (Entity, &ConnectionClientPacketsChannel),
        (With<Connection>, With<ConnectionPacketReader>),
    >,
) {
    for (connection_entity, client_packets_channel) in &query {
        for packet in client_packets_channel.receiver.try_recv() {
            match packet {
                Ok(packet) => {
                    commands.entity(connection_entity).with_children(|parent| {
                        parent.spawn(packet);
                    });
                }
                Err(_) => {}
            }
        }
    }
}
