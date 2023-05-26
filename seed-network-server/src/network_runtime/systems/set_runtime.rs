use bevy::prelude::{Res, ResMut};
use bevy_tokio_runtime::TokioRuntime;
use tokio::runtime::Builder;

use crate::network_settings::NetworkSettings;

pub fn set_runtime(
    mut tokio_runtime: ResMut<TokioRuntime>,
    network_settings: Res<NetworkSettings>,
) -> Result<(), anyhow::Error> {
    let task_pool_threads: usize = network_settings.task_pool_threads;

    let runtime = Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .worker_threads(task_pool_threads)
        .build()?;

    tokio_runtime.new(runtime);

    Ok(())
}
