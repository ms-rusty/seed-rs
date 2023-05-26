use bevy::prelude::States;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum DatabaseServerState {
    #[default]
    Settings, // provavelmente validar a connectionstring
    Loading,
    Running,
}
