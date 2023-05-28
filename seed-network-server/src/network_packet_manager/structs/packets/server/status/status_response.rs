pub struct ServerStatusResponsePacket<'packet> {
    pub version: ServerVersion<'packet>,
    pub players: ServerPlayers,
    pub description: ServerDescription,
}

pub struct ServerVersion<'packet> {
    pub name: &'packet str,
    pub protocol: ServerProtocol,
}

pub struct ServerPlayers {
    pub max: u32,
    pub online: u32,
}

pub struct ServerDescription {}

pub enum ServerProtocol {
    V1_19_4 = 762,
}

// let payload = StatusResponse {
//     version: Version {
//         name: SERVER_NAME,
//         protocol: PROTOCOL_VERSION,
//     },
//     players: Players {
//         max: worker.options().max_players,
//         online: worker.player_count(),
//     },
//     description: Text::from(worker.options().motd.clone()),
//     favicon: worker
//         .options()
//         .favicon
//         .as_ref()
//         .map(Favicon::base64_encoded),
// };
