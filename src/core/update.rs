use std::{
    io::Cursor,
    path::PathBuf,
};

use log::{info, error};
use crate::storage::update_flag;
use self_update::cargo_crate_version;

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
                
                // Refresh the icon
                refresh_app_icon();

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

pub fn refresh_app_icon() {
    #[cfg(target_os = "windows")]
    {
        if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
            let icon_path = PathBuf::from(local_appdata).join("Duetime").join("Duetime.ico");
            let icon_url = "https://raw.githubusercontent.com/Datttta/Duetime/main/assets/Duetime.ico";
            download_and_overwrite(icon_url, &icon_path);
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(home) = std::env::var("HOME") {
            let icon_path = PathBuf::from(home)
                .join(".local/share/icons/hicolor/256x256/Duetime.png");
            let icon_url = "https://raw.githubusercontent.com/Datttta/Duetime/main/assets/Duetime.png";
            download_and_overwrite(icon_url, &icon_path);
        }
    }
}

// Helper to actually perform the download
fn download_and_overwrite(url: &str, path: &PathBuf) {
    info!("Refreshing app icon from {} to {:?}", url, path);
    
    // Ensure the parent directory exists (just in case)
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    // Using reqwest::blocking (adjust to async if your update function is async)
    match reqwest::blocking::get(url) {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(bytes) = response.bytes() {
                    let mut file = match std::fs::File::create(path) {
                        Ok(f) => f,
                        Err(e) => {
                            error!("Failed to open icon file for writing: {}", e);
                            return;
                        }
                    };
                    let mut content = Cursor::new(bytes);
                    if let Err(e) = std::io::copy(&mut content, &mut file) {
                        error!("Failed to write new icon data: {}", e);
                    } else {
                        info!("App icon successfully updated.");
                    }
                }
            } else {
                error!("Failed to download icon: HTTP {}", response.status());
            }
        }
        Err(e) => error!("Network error downloading icon: {}", e),
    }
}
