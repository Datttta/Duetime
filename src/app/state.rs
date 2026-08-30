use ratatui::widgets::{TableState, ListState};

use crate::{
    app::{Popup, SelectedInput, TaskDestination, NewPresetFocus, InboxSelectedFeature, Priority, Panel},
    storage::{current_tasks, known_tasks, preset, inbox},
    vim_text::{InputState, InputMode},
    vim_navigation::NavigationMode,
    tasks::TaskInfo,
    inbox::InboxItemInfo,
    models::{TaskTemplate, Preset, KnownTask},
    move_items::MoveState,
};

use std::time::{Duration, Instant};
pub struct App {
    // Core
    pub running: bool,
    pub popup: Popup,
    pub pending_command: Option<char>,

    // Vim modes
    pub mode: InputMode,
    pub n_mode: NavigationMode,
    pub n_visual_start: Option<usize>,

    // Panels
    pub focused_panel: Panel,
    pub previous_panel: Panel,
    pub is_change: bool,

    // Tasks
    pub tasks: Vec<TaskInfo>,
    pub table_state: TableState,

    pub task_name: InputState,
    pub planned_start: InputState,
    pub planned_end: InputState,
    pub selected_input: SelectedInput,

    // Inbox
    pub inbox_items: Vec<InboxItemInfo>,
    pub inbox_table_state: TableState,

    pub inbox_item: InputState,
    pub inbox_selected_feature: InboxSelectedFeature,
    pub priority: Priority,

    // Presets
    pub presets: Vec<Preset>,
    pub preset_state: ListState,
    pub preset_tasks: Vec<TaskTemplate>,
    pub preset_task_state: ListState,

    pub preset_name: InputState,
    pub edit_preset: Option<usize>,
    pub new_preset_focus: NewPresetFocus,
    pub task_destination: TaskDestination,

    // Known tasks
    pub known_tasks: Vec<KnownTask>,
    pub known_tasks_state: ListState,
    pub known_task_name: InputState,
    pub suggestions: Vec<String>,
    pub selected_suggestion: usize,

    // Misc
    pub move_state: MoveState,
    pub help_scroll: u16,
    pub next_id: u64,

    // Clipboard / notifications
    pub clipboard: Option<arboard::Clipboard>,
    pub status_message: Option<String>,
    pub status_message_until: Option<Instant>,
}

impl App {
    pub fn set_status_message(&mut self, message: String) {
        self.status_message = Some(message);
        self.status_message_until = Some(
            Instant::now() + Duration::from_secs(1)
        );
    }

    pub fn new() -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));

        let mut preset_task_state = ListState::default();
        preset_task_state.select(Some(0));

        let mut preset_state = ListState::default();
        preset_state.select(Some(0));

        let mut known_tasks_state = ListState::default();
        known_tasks_state.select(Some(0));
        
        let mut inbox_table_state = TableState::default();
        inbox_table_state.select(Some(0));

        Self {
            // Core
            pending_command: None,
            popup: Popup::None,
            running: true,

            // Tasks
            task_name: InputState::default(),
            planned_start: InputState::default(),
            planned_end: InputState::default(),
            selected_input: SelectedInput::TaskName,

            tasks: current_tasks::load_current_tasks(),
            table_state,

            // Navigation
            n_mode: NavigationMode::Normal,
            n_visual_start: Some(0),
            mode: InputMode::Insert,

            // Presets
            presets: preset::load_presets(),
            preset_tasks: Vec::new(),
            preset_name: InputState::default(),
            edit_preset: None,
            task_destination: TaskDestination::AddTask,
            new_preset_focus: NewPresetFocus::Name,
            preset_task_state,
            preset_state,

            // Known tasks
            known_task_name: InputState::default(),
            known_tasks_state,
            suggestions: Vec::new(),
            selected_suggestion: 0,
            known_tasks: known_tasks::load_known_tasks(),

            // Move
            move_state: MoveState::default(),

            // Inbox
            inbox_item: InputState::default(),
            inbox_items: inbox::load_inbox(),
            inbox_table_state,
            inbox_selected_feature: InboxSelectedFeature::InboxItemInput,
            priority: Priority::Low,

            // Panel
            focused_panel: Panel::Tasks,
            previous_panel: Panel::Tasks,
            is_change: true,

            // Help
            help_scroll: 0,

            // Other
            next_id: 1,

            // Clipboard / status
            clipboard: arboard::Clipboard::new().ok(),
            status_message: None,
            status_message_until: None,
        }
    }
}
