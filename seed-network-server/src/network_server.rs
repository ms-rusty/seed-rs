use crate::network_channel::{ConnectionEvent, NetworkEventChannel};

pub fn handle_network_event(network_event: NetworkEventChannel) {
    match network_event {
        NetworkEventChannel::ConnectionEvent(connection) => match connection {
            ConnectionEvent::Success((stream, socket_addr)) => {
                println!("stream: {:?}", stream);
                println!("socket_addr: {:?}", socket_addr);
            }
            ConnectionEvent::Failure(error) => {
                println!("error: {:?}", error);
            }
        },
        NetworkEventChannel::DisconnectionEvent(_) => println!("Bye bye!"),
    }
}
