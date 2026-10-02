use log::info;
use self_update::cargo_crate_version;

use std::os::unix::process::CommandExt;

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

                // Create the flag so the new process knows it was updated
                update_flag::create_update_flag(cargo_crate_version!());

                // Exec the newly updated binary to restart seamlessly in-place
                if let Ok(current_exe) = std::env::current_exe() {
                    let err = std::process::Command::new(current_exe).exec();
                    // exec() only returns if it encounters an error (e.g., permission denied)
                    log::error!("Failed to auto-restart after update: {}", err);
                } else {
                    log::error!("Failed to get current executable path for restart");
                }

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
