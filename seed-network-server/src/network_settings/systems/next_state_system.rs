use bevy::prelude::{info, NextState, ResMut};
use seed_network_server_common::NetworkServerState;

pub fn next_state_system(mut next_state: ResMut<NextState<NetworkServerState>>) {
    info!("Next state: NetworkServerState::SettingRuntime");
    next_state.set(NetworkServerState::SettingRuntime);
}
