use bevy::prelude::States;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum AppState {
    #[default]
    StartingSeed,
    StartingDatabaseServer,
    StartingGameWorld,
    StartingNetworkServer,
    Running,
}
