use crate::{
    app::{
        App,
        Popup,
        InboxPopup,
        InboxSelectedInput,
        Priority,
    },
    navigation::vim_navigation::NavigationMode,
    vim_text::InputMode,
    storage::inbox,
    search,
};

pub fn edit_inbox_item(app: &mut App) {
    if let Some(index) = app.inbox_tasks_table_state.selected() {
        let item = &app.inbox_items[index];

        // Load task data into inputs
        app.inbox_item.text = item.input.clone();
        app.priority = item.priority.clone();
        app.inbox_item.cursor = app.inbox_item.text.len();

        app.mode = InputMode::Normal;
        app.popup = Popup::Inbox(InboxPopup::EditInboxItem);
        app.inbox_selected_input = InboxSelectedInput::InboxItemInput;

        app.pending_command = None;
    }
}

pub fn inbox_item_info(app: &mut App) {
    if app.inbox_tasks_table_state.selected().is_some() {
        app.popup = Popup::Inbox(InboxPopup::InfoInboxItem);
    }
}

pub fn inbox_item_add_popup(app: &mut App) {
    app.inbox_item.clear();
    app.priority = Priority::Low;
    app.mode = InputMode::Insert;
    app.inbox_selected_input = InboxSelectedInput::InboxItemInput;
    app.popup = Popup::Inbox(InboxPopup::AddInboxItem);
}

pub fn search_inbox(app: &mut App) {
    search::search_items(
        &mut app.inbox_search,
        &app.inbox_items,
        |item, query| {
            item.input.to_lowercase().contains(query)
        },
    );

    if let Some(&index) = app.inbox_search.matches.first() {
        app.inbox_tasks_table_state.select(Some(index));
    }
}


pub fn delete_inbox_item(app: &mut App) {
    if let Some(current) = app.inbox_tasks_table_state.selected() {
        let (first, last) = if app.n_mode == NavigationMode::Visual {
            if let Some(start) = app.n_visual_start {
                (start.min(current), start.max(current))
            } else {
                (current, current)
            }
        } else {
            (current, current)
        };

        let deleted_count = last - first + 1;
        app.inbox_items.drain(first..=last);

        if app.inbox_items.is_empty() {
            app.inbox_tasks_table_state.select(None);
            *app.inbox_tasks_table_state.offset_mut() = 0;
        } else {
            let new_index = first.min(app.inbox_items.len() - 1);
            app.inbox_tasks_table_state.select(Some(new_index));

            // Only pull the panel up if we aren't already at the default position (offset 0)
            let current_offset = app.inbox_tasks_table_state.offset();
            if current_offset > 0 {
                let new_offset = current_offset.saturating_sub(deleted_count);
                *app.inbox_tasks_table_state.offset_mut() = new_offset;
            }
        }

        app.n_mode = NavigationMode::Normal;
        app.n_visual_start = None;

        inbox::save_inbox(&app.inbox_items).unwrap();
    }
}
