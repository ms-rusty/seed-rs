use bevy::prelude::{
    in_state, App, BuildChildren, Commands, Entity, IntoSystemConfigs, Parent, Plugin, PostStartup,
    PostUpdate, Query, Update,
};
use seed_network_server_common::{
    ClientLoginStartMessage, ClientPingRequestMessage, ClientStatusRequestMessage,
    NetworkServerState, ServerDescription, ServerPingResponseMessage, ServerPlayers,
    ServerPlayersSample, ServerStatusResponseMessage, ServerVersion,
};

pub struct GameClientMessagePlugin;

impl Plugin for GameClientMessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            system.run_if(in_state(NetworkServerState::Running)),
        );
    }
}

fn system(
    mut commands: Commands,
    query_sr: Query<(&Parent, Entity, &ClientStatusRequestMessage)>,
    query_pr: Query<(&Parent, Entity, &ClientPingRequestMessage)>,
    query_ls: Query<(&Parent, Entity, &ClientLoginStartMessage)>,
) {
    for (connection_entity, entity, message) in &query_sr {
        commands.entity(entity).despawn();

        commands
            .entity(connection_entity.get())
            .with_children(|parent| {
                let mut samples = vec![];
                samples.push(ServerPlayersSample {
                    name: "thinkofdeath".to_owned(),
                    id: "4566e69f-c907-48ee-8d71-d7ba5aa00d20".to_owned(),
                });

                parent.spawn(ServerStatusResponseMessage {
                    version: ServerVersion {
                        name: "1.19.4".to_owned(),
                        protocol: 762,
                    },
                    players: ServerPlayers {
                        max: 1,
                        online: 1,
                        sample: samples,
                    },
                    description: ServerDescription {
                        text: "Hello world".to_owned(),
                    },
                    favicon: None,
                    enforces_secure_chat: true,
                    previews_chat: true,
                });
            });
    }

    for (connection_entity, entity, message) in &query_pr {
        commands.entity(entity).despawn();

        commands
            .entity(connection_entity.get())
            .with_children(|parent| {
                parent.spawn(ServerPingResponseMessage {
                    payload: message.payload,
                });
            });
    }

    for (connection_entity, entity, message) in &query_ls {
        commands.entity(entity).despawn();
    }
}
