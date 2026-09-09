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
    TasksTable,
    Inbox,
    Agenda,
    Timers,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Popup {
    None,
    Help,
    TasksTable(TasksTablePopup),
    Inbox(InboxPopup),
    Agenda(AgendaPopup),
    Timers(TimersPopup)
}

#[derive(PartialEq, Debug, Clone)]
pub enum InboxPopup {
    AddInboxItem,
    EditInboxItem,
    InfoInboxItem,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TasksTablePopup {
    AddTask,
    Presets,
    EditTask,
    NewPreset,
    KnownTasks,
    AddKnownTask,
    EditKnownTask(usize),
    TaskInfo,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AgendaPopup {
    AddEvent,
    EditEvent,
    AllEvents,
    EventInfo,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TimersPopup {
    AddTimer,
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
pub enum TaskSelectedInput {
    TaskName,
    PlannedStart,
    PlannedEnd,
}

#[derive(PartialEq)]
pub enum InboxSelectedInput {
    InboxItemInput,
    Priority,
}

#[derive(PartialEq, Debug)]
pub enum AgendaSelectedInput {
    Name,
    Date,
    Time,
    Repeat,
}

#[derive(PartialEq, Debug)]
pub enum TimerSelectedInput {
    Name,
    Duration,
}
