use bevy::prelude::{error, info, Commands, DespawnRecursiveExt, Entity, Query, Res, With};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{
    network_packet_manager::{
        events::ReadPacketEvent,
        packets::{
            ClientHandshakePacket, ClientHandshakingPackets, ClientLoginPackets,
            ClientLoginStartPacket, ClientPingRequestPacket, ClientStatusPackets,
            ClientStatusRequestPacket, NextState, Packet, PacketConnectionEntity,
        },
        resources::ReadPacketChannel,
    },
    shared::{
        Connection, ConnectionHandshakingState, ConnectionLoginState,
        ConnectionPacketHandlerReader, ConnectionStatusState,
    },
};

pub fn handle_read_packets_events(
    mut commands: Commands,
    read_packet_channel: Res<ReadPacketChannel>,
    // handshaking_connections_query: Query<
    //     Entity,
    //     (
    //         With<Connection>,
    //         With<ConnectionPacketHandlerReader>,
    //         With<ConnectionHandshakingState>,
    //     ),
    // >,
    // status_connections_query: Query<
    //     Entity,
    //     (
    //         With<Connection>,
    //         With<ConnectionPacketHandlerReader>,
    //         With<ConnectionStatusState>,
    //     ),
    // >,
    // login_connections_query: Query<
    //     Entity,
    //     (
    //         With<Connection>,
    //         With<ConnectionPacketHandlerReader>,
    //         With<ConnectionLoginState>,
    //     ),
    // >,
) {
    for packet_event in read_packet_channel.receiver.try_iter() {
        commands.spawn((PacketConnectionEntity::new(entity)));

        // let result = match packet_event {
        //     ReadPacketEvent::Success((entity, packet)) => {
        //         if let Ok(entity) = handshaking_connections_query.get(entity) {
        //             let result = handle_client_handshaking_packets(&mut commands, entity, &packet);
        //         }

        //         if let Ok(entity) = status_connections_query.get(entity) {
        //             let result = handle_client_status_packets(&mut commands, entity, &packet);
        //         }

        //         if let Ok(entity) = login_connections_query.get(entity) {
        //             let result = handle_client_login_packets(&mut commands, entity, &packet);
        //         }
        //     }
        //     ReadPacketEvent::Failure((entity, err)) => {
        //         error!("{:?}", err);
        //     }
        // };
    }
}

fn handle_client_handshaking_packets(
    commands: &mut Commands,
    entity: Entity,
    packet: &Packet,
) -> Result<(), anyhow::Error> {
    match FromPrimitive::from_i32(packet.id.value) {
        Some(ClientHandshakingPackets::Handshake) => {
            let request = ClientHandshakePacket::try_from(packet)?;
            info!(target: "systems", "{:?}", request);

            commands
                .entity(entity)
                .remove::<ConnectionHandshakingState>();

            match request.next_state {
                NextState::Status(next_packet) => {
                    commands.entity(entity).insert(ConnectionStatusState);

                    if let Some(next_packet) = next_packet {
                        let request = ClientStatusRequestPacket::try_from(&next_packet)?;
                        info!(target: "systems", "{:?}", request);
                    }
                }
                NextState::Login(next_packet) => {
                    commands.entity(entity).insert(ConnectionLoginState);

                    let request = ClientLoginStartPacket::try_from(&next_packet)?;
                    info!(target: "systems", "{:?}", request);
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn handle_client_status_packets(
    commands: &mut Commands,
    entity: Entity,
    packet: &Packet,
) -> Result<(), anyhow::Error> {
    match FromPrimitive::from_i32(packet.id.value) {
        Some(ClientStatusPackets::PingRequest) => {
            let request = ClientPingRequestPacket::try_from(packet)?;
            info!(target: "systems", "{:?}", request);
        }
        _ => println!("handle_client_status_packets: pacote estranho."),
    }
    Ok(())
}

fn handle_client_login_packets(
    commands: &mut Commands,
    entity: Entity,
    packet: &Packet,
) -> Result<(), anyhow::Error> {
    match FromPrimitive::from_i32(packet.id.value) {
        Some(ClientLoginPackets::EncryptionResponse) => {
            println!("EncryptionResponse.");
        }
        _ => println!("handle_client_login_packets: pacote estranho."),
    }

    Ok(())
}

fn handle_client_play_packets(packet: &Packet) -> Result<(), anyhow::Error> {
    Ok(())
}
