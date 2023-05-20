use std::{fs::File, io::Write, path::Path};

use bevy::prelude::info;

use crate::network_settings::resources::NetworkSettings;

use super::NETWORK_SETTINGS_PATH;

pub fn create_settings_system() -> Result<(), anyhow::Error> {
    info!("Iniciando o sistema para verificar o arquivo settings.");

    if Path::exists(Path::new(NETWORK_SETTINGS_PATH)) {
        info!("O arquivo de settings já existe.");
        return Ok(());
    }

    let mut file = File::create(NETWORK_SETTINGS_PATH)?;

    info!("Criando o arquivo settings padrão.");

    let network_settings = NetworkSettings::default();
    // TODO: anaylize use of toml::to_string
    let settings = toml::to_string(&network_settings)?;
    file.write_all(settings.as_bytes())?;

    info!("Arquivo settings criado com sucesso.");

    Ok(())
}
