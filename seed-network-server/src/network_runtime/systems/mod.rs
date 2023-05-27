pub use next_state_system::next_state_system;
pub use set_runtime_system::set_runtime_system;

mod next_state_system;
mod set_runtime_system;

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, IntoSystem, Startup};
    use bevy_tokio_runtime::TokioRuntime;
    use seed_common::handle_error;

    use crate::network_settings::NetworkSettings;

    use super::set_runtime_system;

    #[test]
    fn test_set_runtime() {
        let mut app = App::new();
        app.init_resource::<NetworkSettings>();
        app.init_resource::<TokioRuntime>();

        app.add_systems(Startup, set_runtime_system.pipe(handle_error));

        app.update();

        let Some(runtime) = app.world.get_resource::<TokioRuntime>() else {
            assert!(false);
            return;
        };

        assert!(runtime.get_runtime().is_some());
    }
}
