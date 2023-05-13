use std::{thread::sleep, time::Duration};

use bevy::prelude::{App, IntoSystemConfigs, OnEnter, OnExit, Plugin};
use seed_common::AppState;

use crate::{
    common::NetworkConnectionEvent, network_connection::NetworkConnectionPlugin,
    network_listener::NetworkListenerPlugin,
};

pub struct NetworkServerPlugin;

impl Plugin for NetworkServerPlugin {
    fn build(&self, app: &mut App) {
        //app.add_plugin(NetworkConnectionPlugin);
        //app.add_plugin(NetworkListenerPlugin);

        //app.add_event::<NetworkConnectionEvent>();

        app.add_systems(OnEnter(AppState::Loading), on_enter_in_loading_system);

        app.add_systems(OnExit(AppState::Loading), on_exit_in_loading_system);

        app.add_systems(OnEnter(AppState::Services), on_enter_in_servies_system);
    }
}

fn on_enter_in_loading_system() {
    println!("on_enter_in_loading_system");
}

fn on_exit_in_loading_system() {
    println!("on_exit_in_loading_system");
}

fn on_enter_in_servies_system() {
    println!("on_enter_in_servies_system");
}
