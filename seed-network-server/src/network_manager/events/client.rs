pub struct ClientId(pub u32);

pub struct Client {
    id: ClientId,
    client_messages_handler: u32,
    server_messages_handler: u32,
}
