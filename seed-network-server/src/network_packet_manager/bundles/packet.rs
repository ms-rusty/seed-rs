use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct PacketBundle {
    connection_entity: PacketConnectionEntity,
    packet: Packet,
}
