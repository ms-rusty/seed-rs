use std::{
    fs::File,
    io::{ErrorKind, Write},
};

use crate::network_settings::resources::NetworkSettings;

use super::NETWORK_SETTINGS_PATH;

pub fn create_settings_system() {
    let mut file = match File::create(NETWORK_SETTINGS_PATH) {
        Ok(file) => file,
        Err(error) => {
            if error.kind() == ErrorKind::AlreadyExists {
                return;
            }
            panic!("Error on create network settings.");
        }
    };

    let network_settings = NetworkSettings::default();
    // TODO: anaylize use of toml::to_string
    let settings = match toml::to_string(&network_settings) {
        Ok(file) => file,
        Err(_) => panic!("Error on serialize network settings."),
    };

    if let Err(_) = file.write_all(settings.as_bytes()) {
        panic!("Error on write network settings.");
    };
}
