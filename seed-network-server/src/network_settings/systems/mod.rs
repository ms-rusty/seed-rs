pub use load_settings::load_settings_system;
pub use next_state::next_state;

mod load_settings;
mod next_state;

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, NextState, State, Update};
    use seed_network_server_common::NetworkServerState;
    use serial_test::serial;

    use crate::network_settings::systems::{
        load_settings::{default_settings, get_or_default, get_settings, remove_settings},
        next_state,
    };

    #[test]
    #[serial]
    fn test_get_or_default() {
        assert!(get_or_default().is_ok());
        assert!(remove_settings().is_ok());
    }

    #[test]
    #[serial]
    fn test_get_settings() {
        assert!(default_settings().is_ok());
        assert!(get_settings().is_ok());
        assert!(remove_settings().is_ok());
    }

    #[test]
    #[serial]
    fn test_default_settings() {
        assert!(default_settings().is_ok());
        assert!(remove_settings().is_ok());
    }

    #[test]
    fn test_next_state_with_state() {
        let mut app = App::new();
        app.add_state::<NetworkServerState>();

        app.add_systems(Update, next_state);

        app.update();

        let next_state = app.world.get_resource::<NextState<NetworkServerState>>();
        assert!(next_state.is_some());

        let Some(next_state) = next_state else {
            assert!(false);
            return;
        };

        let Some(next_state) = next_state.0.clone() else {
            assert!(false);
            return;
        };

        assert_eq!(next_state, NetworkServerState::SettingRuntime);
    }

    #[test]
    fn test_next_state_with_next_state() {
        let mut app = App::new();
        app.add_state::<NetworkServerState>();

        app.add_systems(Update, next_state);

        app.update();
        app.update();

        let state = app.world.get_resource::<State<NetworkServerState>>();
        assert!(state.is_some());

        let Some(state) = state else {
            assert!(false);
            return;
        };

        assert_eq!(state.get(), &NetworkServerState::SettingRuntime);
    }
}
