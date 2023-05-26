use bevy::prelude::States;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum NetworkServerState {
    #[default]
    LoadingSettings,
    SettingRuntime,
    StartingListener,
    Running,
}
