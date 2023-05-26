pub use create_settings::create_settings_system;
pub use load_settings::load_settings_system;
pub use next_state::next_state;

mod create_settings;
mod load_settings;
mod next_state;

const NETWORK_SETTINGS_PATH: &str = "network_server_settings.toml";
