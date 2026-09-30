use log::info;
use self_update::{cargo_crate_version, restart};
// Add these imports:
use std::fs;
use std::path::PathBuf;
use crate::storage::config_location; // Assuming this helps get directory

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

                // 1. Get the data directory (cleanup existing logic if you have it)
                let data_dir = config_location::config_dir(); // Adjust helper path

                // 2. Define the flag file path: duetime_data_dir/update_success.flag
                let flag_path = data_dir.join("update_success.flag");

                // 3. Save the *current* version (which is about to become the 'old' version)
                let current_v_str = cargo_crate_version!();
                
                // Write flag file. If failure, we just log and continue; 
                // the app won't show the popup, but it will still update correctly.
                if let Err(e) = fs::write(&flag_path, current_v_str) {
                    log::error!("Failed to create update flag file at {:?}: {}", flag_path, e);
                } else {
                    info!("Update flag created. Previous version stored: {}", current_v_str);
                }
                
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
