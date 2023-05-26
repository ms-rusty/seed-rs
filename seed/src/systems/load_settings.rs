use std::{fs::File, io::Read};

use crate::resources::{SeedSettings, SEED_SETTINGS_PATH};

pub fn load_settings() -> Result<SeedSettings, anyhow::Error> {
    let mut file = File::open(SEED_SETTINGS_PATH)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let seed_settings = toml::from_str::<SeedSettings>(&content)?;
    Ok(seed_settings)
}
