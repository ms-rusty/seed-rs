pub enum ClientMessage {
    StatusRequest,
    PingRequest { payload: i64 },
}

pub enum ServerMessage<'message> {
    StatusResponse { response: &'message str },
    PingResponse { payload: i64 },
}
