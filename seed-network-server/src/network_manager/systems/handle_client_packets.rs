use bevy::prelude::Res;
use bevy_tokio_runtime::TokioRuntime;
use num_traits::FromPrimitive;

use crate::{
    network::ClientHandshakingPackets,
    network_manager::{
        events::ConnectionState,
        resources::{NetworkChannels, NetworkManager},
    },
};

pub fn handle_client_packets(
    network_channels: Res<NetworkChannels>,
    tokio_runtime: Res<TokioRuntime>,
    network_manager: Res<NetworkManager>,
) {
    for (client_id, packet) in network_channels
        .pending_client_packet_channel
        .receiver
        .try_iter()
    {
        if let Some(client) = network_manager.clients.get(&client_id) {
            match client.connection.state {
                ConnectionState::Handshaking => match FromPrimitive::from_i32(packet.command) {
                    Some(ClientHandshakingPackets::Handshake) => {
                        println!("some...");
                    }
                    _ => {
                        println!("_");
                    }
                },
                ConnectionState::Status => {}
                ConnectionState::Login => {}
                ConnectionState::Play => {}
            }
        }

        // println!("{:?} {:?}", client_id.0, packet.command);

        // let mut reader = PacketReader::from(&packet);
        // println!("protocol_version: {:?}", reader.read_var_int());

        // println!("server_address: {:?}", reader.read_str());

        // println!("server_port: {:?}", reader.read_u16());

        // println!("next_state: {:?}", reader.read_var_int());
    }
}
