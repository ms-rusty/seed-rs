use bevy::prelude::States;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum AppState {
    #[default]
    Loading,
    Running,
}
