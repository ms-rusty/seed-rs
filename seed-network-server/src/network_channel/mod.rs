use bevy::prelude::Resource;
pub use event::{
    ConnectionEvent, NetworkEventChannel, NetworkEventChannelReceiver, NetworkEventChannelSender,
};

pub mod event;

#[derive(Resource)]
pub struct NetworkChannels {
    pub event_sender: NetworkEventChannelSender,
    pub event_receiver: NetworkEventChannelReceiver,
}

impl Default for NetworkChannels {
    fn default() -> Self {
        let message_channel = crossbeam_channel::unbounded::<NetworkEventChannel>();

        Self {
            event_sender: message_channel.0,
            event_receiver: message_channel.1,
        }
    }
}
