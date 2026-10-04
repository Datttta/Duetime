use log::info;
use self_update::cargo_crate_version;
use crate::storage::update_flag;

pub fn check_for_updates() -> bool {
    let status_result = self_update::backends::github::Update::configure()
        .repo_owner("Datttta")
        .repo_name("Duetime")
        .bin_name("Duetime")
        .current_version(cargo_crate_version!())
        .show_output(false)
        .no_confirm(true)
        .build();

    let update_res = match status_result {
        Ok(mut updater) => updater.update(),
        Err(e) => {
            log::warn!("Could not configure updater: {:?}", e);
            return false;
        }
    };

    match update_res {
        Ok(status) => {
            if status.is_updated() {
                info!("Duetime was updated to {}", status.version());

                // Create the flag so the new process knows it was updated
                update_flag::create_update_flag(cargo_crate_version!());

                log::info!("Attempting to restart application...");

                #[cfg(unix)]
                {
                    if let Ok(current_exe) = std::env::current_exe() {
                        let exe_path_str = current_exe.to_string_lossy();
                        let clean_path = exe_path_str.strip_suffix(" (deleted)").unwrap_or(&exe_path_str);
                        
                        use std::os::unix::process::CommandExt;
                        let err = std::process::Command::new(clean_path)
                            .args(std::env::args().skip(1))
                            .exec();
                        log::error!("Failed to auto-restart via exec: {}", err);
                    }
                }

                #[cfg(windows)]
                {
                    // On Windows, spawn a fresh instance before exiting
                    if let Ok(current_exe) = std::env::current_exe() {
                        if let Err(err) = std::process::Command::new(current_exe)
                            .args(std::env::args().skip(1))
                            .spawn() 
                        {
                            log::error!("Failed to spawn new process on Windows: {}", err);
                        }
                    }
                    std::process::exit(0);
                }

                true
            } else {
                info!("Duetime is already up to date");
                false
            }
        }

        Err(error) => {
            log::warn!("Could not check for updates (offline or network error): {:?}", error);
            false
        }
    }
}
