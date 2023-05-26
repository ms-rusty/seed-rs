use std::{fs::File, io::Write, path::Path};

use bevy::prelude::info;

use crate::network_settings::resources::NetworkSettings;

use super::NETWORK_SETTINGS_PATH;

pub fn create_settings_system() -> Result<(), anyhow::Error> {
    let network_settings_path = Path::new(NETWORK_SETTINGS_PATH);
    let settings_exists = Path::exists(network_settings_path);
    if settings_exists {
        return Ok(());
    }

    let network_settings = NetworkSettings::default();
    // TODO: anaylize use of toml::to_string
    let settings = toml::to_string(&network_settings)?;

    let mut file = File::create(NETWORK_SETTINGS_PATH)?;
    file.write_all(settings.as_bytes())?;

    Ok(())
}
