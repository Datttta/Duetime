use std::fs;
use std::path::Path;

use crate::{
    tasks_table::ui::{TaskInfo, TaskInfoData},
    storage::config_location::config_dir,
};

const FILE_NAME: &str = "current_tasks.json";

fn current_tasks_path() -> std::path::PathBuf {
    config_dir().join(FILE_NAME)
}

pub fn save_current_tasks(
    tasks: &[TaskInfo],
) -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<TaskInfoData> =
        tasks.iter().map(TaskInfo::to_data).collect();

    let json = serde_json::to_string_pretty(&data)?;
    let path = current_tasks_path();

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

pub fn load_current_tasks() -> Vec<TaskInfo> {
    let path = current_tasks_path();

    if !path.exists() {
        return Vec::new();
    }

    let json = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    let data: Vec<TaskInfoData> =
        serde_json::from_str(&json).unwrap_or_default();

    data.into_iter()
        .map(TaskInfo::from_data)
        .collect()
}
