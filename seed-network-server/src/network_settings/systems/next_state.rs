use bevy::prelude::{NextState, ResMut};
use seed_network_server_common::NetworkServerState;

pub fn next_state(mut next_state: ResMut<NextState<NetworkServerState>>) {
    next_state.set(NetworkServerState::SettingRuntime);
}
