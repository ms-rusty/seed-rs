use network_channel::{ConnectionEvent, NetworkChannels, NetworkEventChannel};
use tokio::net::TcpListener;

mod network_channel;
mod network_server;

pub struct NetworkServer {
    listener: TcpListener,
    channels: NetworkChannels,
}

impl NetworkServer {
    pub async fn new() -> Self {
        // TODO: handle error.
        let listener = TcpListener::bind("127.0.0.1:65535")
            .await
            .expect("Error on bind TcpListener.");

        Self {
            listener,
            channels: NetworkChannels::default(),
        }
    }

    pub async fn run(self) {
        self.handle_events();
        self.handle_commands();
        self.handle_connections().await;
    }

    fn handle_events(&self) {
        let event_receiver = self.channels.event_receiver.clone();

        tokio::task::spawn(async move {
            loop {
                while let Ok(network_event) = event_receiver.try_recv() {
                    // TODO: improve
                    network_server::handle_network_event(network_event);
                }
            }
        });
    }

    fn handle_commands(&self) {
        // network commands
    }

    async fn handle_connections(self) {
        loop {
            let accept = self.listener.accept().await;

            self.channels
                .event_sender
                .send(NetworkEventChannel::ConnectionEvent(ConnectionEvent::from(
                    accept,
                )))
                .ok();
        }
    }
}
