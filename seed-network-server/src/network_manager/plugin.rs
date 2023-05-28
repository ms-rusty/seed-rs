use bevy::prelude::{App, Plugin};

pub struct NetworkManagerPlugin;

impl Plugin for NetworkManagerPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<NetworkManager>();
        // app.init_resource::<NetworkChannels>();

        // app.add_systems(Startup, (start_listening_system).chain());

        // app.add_systems(
        //     First,
        //     (
        //         handle_connection_event_system,
        //         create_packet_handlers_system,
        //     )
        //         .chain(),
        // );
        // app.add_systems(PreUpdate, (handle_client_packets).chain());
    }
}
