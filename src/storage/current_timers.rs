use std::{
    path::PathBuf,
    fs,
};

use crate::{
    timers::ui::{TimerInfo, TimerInfoData},
    storage::config_location::config_dir,
};

const FILE_NAME: &str = "current_timers.json";

fn current_timers_path() -> PathBuf {
    config_dir().join(FILE_NAME)
}

pub fn save_current_timers(
    timers: &[TimerInfo],
) -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<TimerInfoData> =
        timers.iter().map(TimerInfo::to_data).collect();

    let json = serde_json::to_string_pretty(&data)?;
    let path = current_timers_path();

    // Ensure the config directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write to a temporary file first in the same directory
    let temp_path = path.with_extension("tmp");
    fs::write(&temp_path, &json)?;

    // Atomically rename the temp file to the target file
    fs::rename(&temp_path, &path)?;

    Ok(())
}

pub fn load_current_timers() -> Vec<TimerInfo> {
    let path = current_timers_path();

    if !path.exists() {
        return Vec::new();
    }

    let json = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    let data: Vec<TimerInfoData> =
        serde_json::from_str(&json).unwrap_or_default();

    data.into_iter()
        .map(TimerInfo::from_data)
        .collect()
}
