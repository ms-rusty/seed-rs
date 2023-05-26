use bevy::prelude::{Res, ResMut};
use bevy_tokio_runtime::TokioRuntime;
use tokio::runtime::Builder;

use crate::network_settings::NetworkSettings;

pub fn set_runtime(
    mut tokio_runtime: ResMut<TokioRuntime>,
    network_settings: Res<NetworkSettings>,
) -> Result<(), anyhow::Error> {
    let runtime_threads: usize = network_settings.runtime_threads;

    let runtime = Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .worker_threads(runtime_threads)
        .build()?;

    tokio_runtime.new(runtime);

    Ok(())
}
