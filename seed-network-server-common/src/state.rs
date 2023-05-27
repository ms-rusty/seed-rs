use bevy::prelude::States;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum NetworkServerState {
    #[default]
    Pending,
    LoadingSettings,
    SettingRuntime,
    StartingListener,
    AcceptConnections,
    StartingConnectionListener,
    Running,
}
