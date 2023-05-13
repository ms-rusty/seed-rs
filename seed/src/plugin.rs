use bevy::{
    app::RunFixedUpdateLoop,
    prelude::{
        App, First, FixedUpdate, IntoSystemConfigs, Last, Main, NextState, OnEnter, Plugin,
        PostStartup, PostUpdate, PreStartup, PreUpdate, ResMut, Startup, StateTransition, Update,
    },
};
use seed_common::AppState;

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), app_state_system);

        app.add_systems(PreStartup, || println!("PreStartup"));
        app.add_systems(Startup, || println!("Startup"));
        app.add_systems(PostStartup, || println!("PostStartup"));
        app.add_systems(First, || println!("First"));
        app.add_systems(PreUpdate, || println!("PreUpdate"));
        app.add_systems(StateTransition, || println!("StateTransition"));
        app.add_systems(RunFixedUpdateLoop, || println!("RunFixedUpdateLoop"));
        app.add_systems(FixedUpdate, || println!("FixedUpdate"));
        app.add_systems(Update, || println!("Update"));
        app.add_systems(PostUpdate, || println!("PostUpdate"));
        app.add_systems(Last, || println!("Last"));
    }
}

fn app_state_system(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Services);
}
