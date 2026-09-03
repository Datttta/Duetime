use std::fs;

use crate::agenda::ui::{AgendaEventData, AgendaEvent};

use crate::storage::config_location::config_dir;

const FILE_NAME: &str = "Agenda.json";

fn agenda_path() -> std::path::PathBuf {
    config_dir().join(FILE_NAME)
}

pub fn save_agenda(
    events: &[AgendaEvent],
) -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<AgendaEventData> =
        events.iter().map(AgendaEvent::to_data).collect();

    let json = serde_json::to_string_pretty(&data)?;
    fs::write(agenda_path(), json)?;

    Ok(())
}

pub fn load_agenda() -> Vec<AgendaEvent> {
    let path = agenda_path();

    if !path.exists() {
        return Vec::new();
    }

    let json = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    let data: Vec<AgendaEventData> =
        serde_json::from_str(&json).unwrap_or_default();

    data.into_iter()
        .map(AgendaEvent::from_data)
        .collect()
}
