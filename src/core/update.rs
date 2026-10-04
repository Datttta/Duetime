use log::info;
use self_update::cargo_crate_version;
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

                if let Ok(current_exe) = std::env::current_exe() {
                    let exe_path_str = current_exe.to_string_lossy();
                    // Strip " (deleted)" if present in the path string (Linux specific)
                    let clean_path = exe_path_str.strip_suffix(" (deleted)").unwrap_or(&exe_path_str);
                    let target_path = std::path::Path::new(clean_path);

                    log::info!("Attempting to restart application on path: {:?}", target_path);

                    #[cfg(unix)]
                    {
                        use std::os::unix::process::CommandExt;
                        let err = std::process::Command::new(target_path)
                            .args(std::env::args().skip(1))
                            .exec();

                        log::error!("Failed to auto-restart via exec: {}", err);
                    }

                    #[cfg(windows)]
                    {
                        let spawn_result = std::process::Command::new(target_path)
                            .args(std::env::args().skip(1))
                            .spawn();

                        match spawn_result {
                            Ok(_) => {
                                info!("Successfully spawned new process. Exiting current process.");
                                // Safely exit the old process so the new executable file can fully take over
                                std::process::exit(0);
                            }
                            Err(err) => {
                                log::error!("Failed to spawn new process on Windows: {}", err);
                            }
                        }
                    }
                } else {
                    log::error!("Failed to get current executable path for restart");
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
