use systems::{get_seed_settings, run_app};

mod plugin;
mod resources;
mod systems;

fn main() -> Result<(), anyhow::Error> {
    let settings = get_seed_settings()?;
    run_app(settings);
    Ok(())
}
