use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Preset {
    pub id: u64,
    pub name: String,
    pub tasks: Vec<TaskTemplate>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TaskTemplate {
    pub id: u64,
    pub name: String,
    pub planned_start: Option<String>,
    pub planned_end: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct KnownTask {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InboxItem {
    pub id: u64,
    pub name: String,
}


