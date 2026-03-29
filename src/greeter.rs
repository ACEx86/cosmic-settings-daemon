use std::time::Duration;

use cosmic_config::CosmicConfigEntry;
use cosmic_settings_daemon_config::greeter;

pub fn sync_with_greeter() -> anyhow::Result<()> {
    log::trace!("syncing with greeter...");
    log::trace!("applied greeter state");

    Ok(())
}
