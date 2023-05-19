use std::{fs::File, io::Read};

use bevy::prelude::{info, Commands};

use crate::network_settings::resources::NetworkSettings;

use super::NETWORK_SETTINGS_PATH;

pub fn load_settings_system(mut commands: Commands) -> Result<(), anyhow::Error> {
    info!("Iniciando o sistema para carregar o arquivo settings.");

    let mut file = File::open(NETWORK_SETTINGS_PATH)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    info!("Arquivo settings carregado com sucesso.");

    let network_settings = toml::from_str::<NetworkSettings>(&content)?;

    commands.insert_resource(network_settings);

    info!("Recurso settings criado com sucesso.");

    Ok(())
}
