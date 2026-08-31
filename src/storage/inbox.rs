use std::fs;

use crate::inbox::ui::{InboxItemInfoData, InboxItemInfo};

use crate::storage::config_location::config_dir;

const FILE_NAME: &str = "Inbox.json";

fn inbox_path() -> std::path::PathBuf {
    config_dir().join(FILE_NAME)
}

pub fn save_inbox(
    inbox_items: &[InboxItemInfo],
    ) -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<InboxItemInfoData> = inbox_items.iter().map(InboxItemInfo::to_data).collect();

    let json = serde_json::to_string_pretty(&data)?;
    fs::write(inbox_path(), json)?;
    
    Ok(())
}


pub fn load_inbox() -> Vec<InboxItemInfo> {
    let path = inbox_path();

    if !path.exists() {
        return Vec::new();
    }

    let json = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    let data: Vec<InboxItemInfoData> =
        serde_json::from_str(&json).unwrap_or_default();

    data.into_iter()
        .map(InboxItemInfo::from_data)
        .collect()
}

