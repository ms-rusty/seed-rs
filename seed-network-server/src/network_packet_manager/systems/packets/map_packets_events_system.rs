use bevy::prelude::{info, BuildChildren, Commands, Res};

use crate::network_packet_manager::{events::ReadPacketEvent, resources::ReadPacketChannel};

pub fn map_packets_events_system(
    mut commands: Commands,
    read_packet_channel: Res<ReadPacketChannel>,
) {
    for packet_event in read_packet_channel.receiver.try_iter() {
        match packet_event {
            ReadPacketEvent::Success((connection_entity, packet)) => {
                commands.entity(connection_entity).with_children(|parent| {
                    parent.spawn(packet);
                });
            }
            ReadPacketEvent::Failure((connection_entity, err)) => {
                // commands.entity(connection_entity).insert(bundle);
            }
        };
    }
}
