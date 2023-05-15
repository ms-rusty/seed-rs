use std::{fs::File, io::Read};

use bevy::prelude::Commands;

use crate::network_settings::resources::NetworkSettings;

use super::NETWORK_SETTINGS_PATH;

pub fn load_settings_system(mut commands: Commands) {
    let mut file = match File::open(NETWORK_SETTINGS_PATH) {
        Ok(file) => file,
        Err(_) => panic!("Error on open network settings."),
    };

    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content) {
        panic!("Error on read content from network settings.");
    };

    let network_settings = match toml::from_str::<NetworkSettings>(&content) {
        Ok(settings) => settings,
        Err(_) => panic!("Error on deserialie network settings."),
    };

    commands.insert_resource(network_settings);
}
