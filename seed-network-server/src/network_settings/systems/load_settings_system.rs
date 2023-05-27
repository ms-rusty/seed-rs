use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use bevy::prelude::Commands;

use crate::network_settings::resources::{NetworkSettings, FILE_NAME};

pub fn load_settings_system(mut commands: Commands) -> Result<(), anyhow::Error> {
    let network_settings = get_or_default()?;
    commands.insert_resource(network_settings);

    Ok(())
}

pub fn get_or_default() -> Result<NetworkSettings, anyhow::Error> {
    let file_path = Path::new(FILE_NAME);
    let file_exists = Path::exists(file_path);
    if file_exists {
        get_settings()
    } else {
        default_settings()
    }
}

pub fn get_settings() -> Result<NetworkSettings, anyhow::Error> {
    let mut file: File = File::open(FILE_NAME)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let network_settings = toml::from_str(&content)?;
    Ok(network_settings)
}

pub fn default_settings() -> Result<NetworkSettings, anyhow::Error> {
    let default_network_settings = NetworkSettings::default();
    let settings = toml::to_string(&default_network_settings)?;

    let mut file = File::create(FILE_NAME)?;
    file.write_all(settings.as_bytes())?;

    Ok(default_network_settings)
}

#[cfg(test)]
pub fn remove_settings() -> Result<(), std::io::Error> {
    std::fs::remove_file(FILE_NAME)
}
