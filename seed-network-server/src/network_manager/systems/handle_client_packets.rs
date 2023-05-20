use bevy::prelude::{Res, ResMut};
use bevy_tokio_runtime::TokioRuntime;
use num_traits::FromPrimitive;

use crate::{
    network::{
        ClientHandshakingPackets, ClientStatusPackets, HandshakeState, Packet,
        PacketClientHandshake, PacketClientStatusRequest,
    },
    network_manager::{
        events::{Client, ConnectionState},
        resources::{NetworkChannels, NetworkManager},
    },
};

pub fn handle_client_packets(
    network_channels: Res<NetworkChannels>,
    tokio_runtime: Res<TokioRuntime>,
    mut network_manager: ResMut<NetworkManager>,
) {
    for (client_id, packet) in network_channels
        .pending_client_packet_channel
        .receiver
        .try_iter()
    {
        let Some(client) = network_manager.clients.get_mut(&client_id) else {
            continue;
        };

        let result = match client.connection.state {
            ConnectionState::Handshaking => handle_client_handshaking_packets(client, &packet),
            ConnectionState::Status => handle_client_status_packets(&packet),
            ConnectionState::Login => handle_client_login_packets(&packet),
            ConnectionState::Play => handle_client_play_packets(&packet),
        };
    }
}

fn handle_client_handshaking_packets(
    client: &mut Client,
    packet: &Packet,
) -> Result<(), anyhow::Error> {
    match FromPrimitive::from_i32(packet.command) {
        Some(ClientHandshakingPackets::Handshake) => {
            let request = PacketClientHandshake::try_from(packet)?;

            let next_state = match request.next_state {
                HandshakeState::Status => ConnectionState::Status,
                HandshakeState::Login => ConnectionState::Login,
            };
            client.connection.state = next_state;
        }
        _ => {}
    }

    Ok(())
}

fn handle_client_status_packets(packet: &Packet) -> Result<(), anyhow::Error> {
    match FromPrimitive::from_i32(packet.command) {
        Some(ClientStatusPackets::StatusRequest) => {
            let request = PacketClientStatusRequest::try_from(packet)?;
        }
        _ => {}
    }
    Ok(())
}

fn handle_client_login_packets(packet: &Packet) -> Result<(), anyhow::Error> {
    Ok(())
}

fn handle_client_play_packets(packet: &Packet) -> Result<(), anyhow::Error> {
    Ok(())
}
