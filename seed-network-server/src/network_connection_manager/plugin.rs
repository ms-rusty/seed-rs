use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, Plugin, Res, ResMut, Update};
use bevy_tokio_runtime::TokioRuntime;
use seed_network_server_common::NetworkServerState;

use super::{resources::ListenerTask, systems::set_listener_system};

pub struct NetworkConnectionManagerPlugin;

impl Plugin for NetworkConnectionManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(NetworkServerState::StartingConnectionListener),
            set_listener_system,
        );

        app.add_systems(
            Update,
            change.run_if(in_state(NetworkServerState::StartingConnectionListener)),
        );
    }
}

fn change(mut listener_task: ResMut<ListenerTask>) {}
