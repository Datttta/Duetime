use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Priority {
    #[default]
    Low,
    Medium,
    High,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Panel {
    Tasks,
    Inbox,
}

#[derive(PartialEq, Debug)]
pub enum Popup {
    None,
    Help,
    Tasks(TasksPopup),
    Inbox(InboxPopup),
}

#[derive(PartialEq, Debug)]
pub enum InboxPopup {
    AddInboxItem,
    EditInboxItem,
    InfoInboxItem,
}

#[derive(PartialEq, Debug)]
pub enum TasksPopup {
    AddTask,
    Presets,
    EditTask,
    NewPreset,
    KnownTasks,
    AddKnownTask,
    EditKnownTask(usize),
    TaskInfo,
}

#[derive(PartialEq)]
pub enum NewPresetFocus {
    Name,
    Tasks,
}

pub enum TaskDestination {
    AddTask,
    Preset,
    EditTask(usize),
    EditPresetTask(usize),
}

#[derive(PartialEq)]
pub enum SelectedInput {
    TaskName,
    PlannedStart,
    PlannedEnd,
}

#[derive(PartialEq)]
pub enum InboxSelectedFeature {
    InboxItemInput,
    Priority,
}
