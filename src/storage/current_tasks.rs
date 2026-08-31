use std::fs;
use directories::ProjectDirs;

use crate::tasks::ui::{TaskInfoData, TaskInfo};

const FILE_NAME: &str = "current_tasks.json";

fn current_tasks_path() -> std::path::PathBuf {
    let proj_dirs = ProjectDirs::from("", "", "Duetime")
        .expect("Could not determine config directory");

    let config_dir = proj_dirs.config_dir();

    fs::create_dir_all(config_dir)
        .expect("Could not create config directory");

    config_dir.join(FILE_NAME)
}

pub fn save_current_tasks(
    tasks: &[TaskInfo],
) -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<TaskInfoData> = tasks.iter().map(TaskInfo::to_data).collect();

    let json = serde_json::to_string_pretty(&data)?;
    fs::write(current_tasks_path(), json)?;
    
    Ok(())
}

pub fn load_current_tasks() -> Vec<TaskInfo> {
    let path = current_tasks_path();

    if !path.exists() {
        return Vec::new();
    }

    let json = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    let data: Vec<TaskInfoData> =
        serde_json::from_str(&json).unwrap_or_default();

    data.into_iter()
        .map(TaskInfo::from_data)
        .collect()
}
