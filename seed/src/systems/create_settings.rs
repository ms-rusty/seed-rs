use std::{fs::File, io::Write, path::Path};

use crate::resources::{SeedSettings, SEED_SETTINGS_PATH};

pub fn create_settings_system() -> Result<(), anyhow::Error> {
    let seed_settings_path = Path::new(SEED_SETTINGS_PATH);
    let settings_exists = Path::exists(seed_settings_path);
    if settings_exists {
        return Ok(());
    }

    let seed_settings = SeedSettings::default();
    let settings = toml::to_string(&seed_settings)?;

    let mut file = File::create(SEED_SETTINGS_PATH)?;
    file.write_all(settings.as_bytes())?;

    Ok(())
}
