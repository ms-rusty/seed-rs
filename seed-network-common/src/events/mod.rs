use uuid::Uuid;

pub enum ClientHandshakingEvent {}

pub enum ClientStatusEvent {
    StatusRequest,
    PingRequest { payload: i64 },
}

pub enum ClientLoginEvent<'event> {
    LoginStart {
        username: &'event str,
        has_player_uuid: bool,
        player_uuid: Option<Uuid>,
    },
}
