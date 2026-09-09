use std::fs;
use std::path::PathBuf;

use crate::timers::ui::{TimerInfo, TimerInfoData};

const FILE_NAME: &str = "current_timers.json";

fn current_timers_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("storage")
        .join(FILE_NAME)
}

pub fn save_current_timers(
    timers: &[TimerInfo],
) -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<TimerInfoData> =
        timers.iter().map(TimerInfo::to_data).collect();

    let json = serde_json::to_string_pretty(&data)?;
    fs::write(current_timers_path(), json)?;

    Ok(())
}

pub fn load_current_timers() -> Vec<TimerInfo> {
    let path = current_timers_path();

    if !path.exists() {
        return Vec::new();
    }

    let json = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    let data: Vec<TimerInfoData> =
        serde_json::from_str(&json).unwrap_or_default();

    data.into_iter()
        .map(TimerInfo::from_data)
        .collect()
}
