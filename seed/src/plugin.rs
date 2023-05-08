use bevy::prelude::{App, Plugin};

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(|| println!("Starting seed plugin."));
    }
}

// Tokio Runtime configuration.
// fn build_tokio_runtime() -> tokio::runtime::Runtime {
//     app.add_plugin(bevy_tokio_tasks::TokioTasksPlugin {
//         make_runtime: Box::from(|| build_tokio_runtime()),
//     });
//     tokio::runtime::Builder::new_multi_thread()
//         .worker_threads(2)
//         .enable_all()
//         .build()
//         .expect("Error on build the Tokio Runtime.")
// }
