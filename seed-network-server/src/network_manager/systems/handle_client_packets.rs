use bevy::prelude::{info, Commands, Entity, EventWriter, Query, Res, ResMut, With};
use bevy_tokio_runtime::TokioRuntime;
use num_traits::FromPrimitive;

use crate::network_manager::{
    events::{
        Client, Connection, ConnectionHandshakingState, ConnectionLoginState, ConnectionState,
        ConnectionStatusState, PacketHandler,
    },
    packets::{
        ClientHandshakePacket, ClientHandshakingPackets, ClientLoginPackets,
        ClientLoginStartPacket, ClientPingRequestPacket, ClientStatusPackets,
        ClientStatusRequestPacket, NextState, Packet,
    },
    resources::{NetworkChannels, NetworkManager},
};

pub fn handle_client_packets(
    mut commands: Commands,
    network_channels: Res<NetworkChannels>,
    handshaking_clients_query: Query<
        Entity,
        (With<ConnectionHandshakingState>, With<PacketHandler>),
    >,
    status_clients_query: Query<Entity, (With<ConnectionStatusState>, With<PacketHandler>)>,
    login_clients_query: Query<Entity, (With<ConnectionLoginState>, With<PacketHandler>)>,
) {
    for (entity, packet) in network_channels
        .pending_client_packet_channel
        .receiver
        .try_iter()
    {
        if handshaking_clients_query.contains(entity) {
            handle_client_handshaking_packets(&mut commands, entity, &packet);
        }

        if status_clients_query.contains(entity) {
            handle_client_status_packets(&packet);
        }

        if login_clients_query.contains(entity) {
            handle_client_login_packets(&packet);
        }

        // let result = match connection_state {
        //     ConnectionState::Handshaking => handle_client_handshaking_packets(&packet),
        //     ConnectionState::Status => handle_client_status_packets(&packet),
        //     ConnectionState::Login => handle_client_login_packets(&packet),
        //     ConnectionState::Play => handle_client_play_packets(&packet),
        // };
        //}
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

fn handle_client_status_packets(packet: &Packet) -> Result<(), anyhow::Error> {
    match FromPrimitive::from_i32(packet.id.value) {
        Some(ClientStatusPackets::PingRequest) => {
            let request = ClientPingRequestPacket::try_from(packet)?;
            info!(target: "systems", "{:?}", request);
        }
        _ => println!("handle_client_status_packets: pacote estranho."),
    }
    Ok(())
}

fn handle_client_login_packets(packet: &Packet) -> Result<(), anyhow::Error> {
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
