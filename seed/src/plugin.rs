use bevy::prelude::{
    in_state, App, IntoSystemConfigs, NextState, OnEnter, Plugin, Res, ResMut, Startup, State,
    Update,
};

use seed_common::AppState;
use seed_database_server_common::DatabaseServerState;
use seed_game_world_common::GameWorldState;
use seed_network_server_common::NetworkServerState;

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
        app.add_state::<DatabaseServerState>();
        app.add_state::<GameWorldState>();
        app.add_state::<NetworkServerState>();

        app.add_systems(OnEnter(AppState::Loading), init_database_server);
        app.add_systems(OnEnter(DatabaseServerState::Running), init_game_world);
        app.add_systems(OnEnter(GameWorldState::Running), init_network_server);
        // app.add_systems(OnEnter(NetworkServerState::Running), running_app);

        // app.add_systems(
        //     Update,
        //     haha.run_if(in_state(DatabaseServerState::LoadingSettings)),
        // );
        // app.add_systems(
        //     Update,
        //     hehe.run_if(in_state(GameWorldState::LoadingSettings)),
        // );
        // app.add_systems(
        //     Update,
        //     hihi.run_if(in_state(NetworkServerState::LoadingSettings)),
        // );
    }
}

fn haha(mut next_state: ResMut<NextState<DatabaseServerState>>) {
    println!("haha");
    next_state.set(DatabaseServerState::Running);
}

fn hehe(mut next_state: ResMut<NextState<GameWorldState>>) {
    println!("hehe");
    next_state.set(GameWorldState::Running);
}

fn hihi(mut next_state: ResMut<NextState<NetworkServerState>>) {
    println!("hihi");
    next_state.set(NetworkServerState::Running);
}

pub fn init_database_server(mut next_state: ResMut<NextState<DatabaseServerState>>) {
    next_state.set(DatabaseServerState::Running);
}

fn init_game_world(mut next_state: ResMut<NextState<GameWorldState>>) {
    next_state.set(GameWorldState::Running);
}

fn init_network_server(mut next_state: ResMut<NextState<NetworkServerState>>) {
    next_state.set(NetworkServerState::LoadingSettings);
}

fn running_app(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Running);
}

// app.add_systems(PreStartup, || println!("PreStartup"));
// app.add_systems(Startup, || println!("Startup"));
// app.add_systems(PostStartup, || println!("PostStartup"));
// app.add_systems(First, || println!("First"));
// app.add_systems(PreUpdate, || println!("PreUpdate"));
// app.add_systems(StateTransition, || println!("StateTransition"));
// app.add_systems(RunFixedUpdateLoop, || println!("RunFixedUpdateLoop"));
// app.add_systems(FixedUpdate, || println!("FixedUpdate"));
// app.add_systems(Update, || println!("Update"));
// app.add_systems(PostUpdate, || println!("PostUpdate"));
// app.add_systems(Last, || println!("Last"));
