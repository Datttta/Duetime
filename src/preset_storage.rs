use std::fs;
use directories::ProjectDirs;

use crate::presets::Preset;

const FILE_NAME: &str = "presets.json";

fn presets_path() -> std::path::PathBuf {
    let proj_dirs = ProjectDirs::from("", "", "Duetime")
        .expect("Could not determine config directory");

    let config_dir = proj_dirs.config_dir();

    fs::create_dir_all(config_dir)
        .expect("Could not create config directory");

    config_dir.join(FILE_NAME)
}

pub fn save_preset(
    presets: &[Preset],
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(presets)?;
    fs::write(presets_path(), json)?;
    Ok(())
}

pub fn load_presets() -> Vec<Preset> {
    let path = presets_path();

    if !path.exists() {
        return Vec::new();
    }

    let json = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    serde_json::from_str(&json).unwrap_or_default()
}
