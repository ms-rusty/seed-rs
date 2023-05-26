use crate::resources::SeedSettings;

use super::{create_settings_system, load_settings};

pub fn get_seed_settings() -> Result<SeedSettings, anyhow::Error> {
    create_settings_system()?;
    load_settings()
}
