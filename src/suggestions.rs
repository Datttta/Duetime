use crate::models::KnownTask;

pub fn task_name_suggestions<'a>(
    known_tasks: &'a [KnownTask],
    input: &str,
) -> Vec<&'a KnownTask> {
    let input = input.to_lowercase();

    known_tasks
        .iter()
        .filter(|task| task.name.to_lowercase().starts_with(&input))
        .collect()
}
