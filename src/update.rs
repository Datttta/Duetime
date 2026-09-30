use log::info;
use self_update::{cargo_crate_version, restart};

pub fn check_for_updates() -> bool {
    let result = self_update::backends::github::Update::configure()
        .repo_owner("Datttta")
        .repo_name("Duetime")
        .bin_name("Duetime")
        .current_version(cargo_crate_version!())
        .show_output(false) // Shows download progress bar in console before TUI launches
        .no_confirm(true)
        .build()
        .and_then(|update| update.update());

    match result {
        Ok(status) => {
            if status.is_updated() {
                info!("Duetime was updated to {}", status.version());
                let _ = restart::restart();
                true
            } else {
                info!("Duetime is already up to date");
                info!("Current Duetime version: {}", cargo_crate_version!());
                false
            }
        }

        Err(error) => {
            log::error!("Update failed: {:?}", error);
            false
        }
    }
}
