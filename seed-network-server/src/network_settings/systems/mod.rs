pub use create_settings::create_settings_system;
pub use load_settings::load_settings_system;

mod create_settings;
mod load_settings;

const NETWORK_SETTINGS_PATH: &str = "network_server_settings.toml";
