use std::collections::HashSet;
use crate::models::{KnownTask, Preset};
use ratatui::{
    style::{Style},
    widgets::{List, ListItem},
};

pub fn task_name_suggestions(
    known_tasks: &[KnownTask],
    presets: &[Preset],
    input: &str,
) -> Vec<String> {
    if input.is_empty() {
        return Vec::new();
    }

    let input = input.to_lowercase();
    let mut seen = HashSet::new();

    known_tasks
        .iter()
        .map(|task| task.name.as_str())
        .chain(
            presets
                .iter()
                .flat_map(|preset| preset.tasks.iter())
                .map(|task| task.name.as_str()),
        )
        .filter(|name| name.to_lowercase().starts_with(&input))
        .filter(|name| seen.insert(name.to_lowercase()))
        .take(8)
        .map(str::to_owned)
        .collect()
}

pub fn task_name_list(
    known_tasks: &[KnownTask],
    presets: &[Preset],
    input: &str,
    selected: usize,
) -> List<'static> {
    let items: Vec<ListItem> = task_name_suggestions(
        known_tasks,
        presets,
        input,
    )
    .into_iter()
    .enumerate()
    .map(|(i, name)| {
        let item = ListItem::new(name);

        if i == selected {
            item.style(Style::default().reversed())
        } else {
            item
        }
    })
    .collect();

    List::new(items)
}
