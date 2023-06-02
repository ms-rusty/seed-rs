use bevy::{
    ecs::event,
    prelude::{in_state, App, EventReader, IntoSystemConfigs, Plugin, Update},
};
use seed_network_server_common::{NetworkServerState, ServerMessage};

use super::{
    resources::ReadPacketChannel,
    systems::{
        handshake_system, handshaking_packet_system, init_connection_packet_handler_reader_system,
        init_connection_packet_handler_writer_system, login_packet_system,
        map_packets_events_system, ping_request_system, status_packet_system,
        status_request_system,
    },
};

pub struct NetworkPacketManagerPlugin;

impl Plugin for NetworkPacketManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReadPacketChannel>();

        app.add_systems(
            Update,
            (
                init_connection_packet_handler_reader_system,
                init_connection_packet_handler_writer_system,
                map_packets_events_system,
                handshaking_packet_system,
                login_packet_system,
                status_packet_system,
                handshake_system,
                status_request_system,
                ping_request_system,
            )
                .chain()
                .run_if(in_state(NetworkServerState::Running)),
        );
    }
}

fn system(mut event_reader: EventReader<ServerMessage<'static>>) {
    for event in event_reader.iter() {
        // tokio_runtime.spawn_task(async move {});
    }
}
