use bevy::prelude::{App, Plugin, ResMut};
use bevy_tokio_runtime::TokioRuntime;

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        //app.add_startup_system(start_tokio_runtime);
        // .in_schedule(OnExit(AppState::Loading))
    }
}

fn start_tokio_runtime(mut tokio_runtime: ResMut<TokioRuntime>) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .worker_threads(2)
        .build()
        .unwrap();

    tokio_runtime.new(runtime);
}
