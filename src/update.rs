use log::info;
use self_update::{cargo_crate_version, restart};

use crate::storage::update_flag;

pub fn check_for_updates() -> bool {
    let result = self_update::backends::github::Update::configure()
        .repo_owner("Datttta")
        .repo_name("Duetime")
        .bin_name("Duetime")
        .current_version(cargo_crate_version!())
        .show_output(false)
        .no_confirm(true)
        .build()
        .and_then(|update| update.update());

    match result {
        Ok(status) => {
            if status.is_updated() {
                info!("Duetime was updated to {}", status.version());

                // Store current version string into flag file via storage module
                update_flag::create_update_flag(cargo_crate_version!());

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
