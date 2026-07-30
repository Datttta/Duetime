use std::fs;
use std::path::Path;

use crate::presets::Preset;

const FILE_NAME: &str = "presets.json";

pub fn save_preset(presets: &[Preset]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(presets)?;

    fs::write(FILE_NAME, json)?;

    Ok(())
}

pub fn load_presets() -> Vec<Preset> {
    let path = Path::new(FILE_NAME);

    if !path.exists() {
        return Vec::new();
    }

    let json = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    serde_json::from_str(&json).unwrap_or_default()
}
