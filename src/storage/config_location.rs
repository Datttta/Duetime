use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;

pub fn config_dir() -> PathBuf {
    let proj_dirs = ProjectDirs::from("", "", "Duetime")
        .expect("Could not determine config directory");

    let config_dir = proj_dirs.config_dir();

    if config_dir.exists() && !config_dir.is_dir() {
        panic!(
            "Config path exists but is not a directory: {}",
            config_dir.display()
        );
    }

    fs::create_dir_all(config_dir)
        .expect("Could not create config directory");

    config_dir.to_path_buf()
}
