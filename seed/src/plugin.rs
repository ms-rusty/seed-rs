use bevy::prelude::{App, Plugin};

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        // Tokio Runtime configuration.
        app.add_plugin(bevy_tokio_tasks::TokioTasksPlugin {
            make_runtime: Box::from(build_tokio_runtime()),
        });
    }
}

fn build_tokio_runtime() -> impl Fn() -> tokio::runtime::Runtime {
    || {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("Error on build the Tokio Runtime.")
    }
}
