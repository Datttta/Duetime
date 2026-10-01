use std::env;
use std::fs;
use std::path::PathBuf;
use log::{info, error};

// Prefixing with the app name prevents collisions in shared system temp folders
const FILE_NAME: &str = "duetime_update_success.flag";

fn flag_path() -> PathBuf {
    env::temp_dir().join(FILE_NAME)
}

/// Writes the current version to the OS temporary directory before restarting.
pub fn create_update_flag(current_version: &str) {
    let path = flag_path();

    if let Err(e) = fs::write(&path, current_version) {
        error!("Failed to create update flag file at {:?}: {}", path, e);
    } else {
        info!("Update flag created at {:?}. Previous version stored: {}", path, current_version);
    }
}

/// Checks if an update flag exists in the OS temp directory, reads the old version,
/// deletes the file, and returns the version string.
pub fn detect_and_clean_update_flag() -> Option<String> {
    let path = flag_path();

    if !path.exists() {
        return None;
    }

    match fs::read_to_string(&path) {
        Ok(old_version_content) => {
            info!("Update flag detected at {:?}. Previous version: {}", path, old_version_content);

            if let Err(e) = fs::remove_file(&path) {
                error!("Failed to delete update flag file at {:?}: {}", path, e);
                None
            } else {
                info!("Update flag file removed successfully.");
                Some(old_version_content.trim().to_string())
            }
        }
        Err(e) => {
            error!("Failed to read update flag file at {:?}: {}", path, e);
            let _ = fs::remove_file(&path);
            None
        }
    }
}
