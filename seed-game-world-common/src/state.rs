use bevy::prelude::States;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum GameWorldState {
    #[default]
    Pending,
    LoadingSettings,
    Running,
}
