use crate::cli::{SortDirection, SortItem};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub done: bool,
}

// TODO: remove reference from optionsal
pub fn list_tasks(
    tasks: &[Task],
    all: bool,
    done: bool,
    sort_item: &Option<SortItem>,
    sort_direction: &SortDirection,
) -> Vec<Task> {
    let mut filtered = if all {
        tasks.to_vec()
    } else if done {
        tasks.iter().filter(|t| t.done).cloned().collect()
    } else {
        tasks.iter().filter(|t| !t.done).cloned().collect()
    };

    if let Some(field) = sort_item {
        filtered.sort_by(|t1, t2| {
            let ordering = match field {
                SortItem::Id => t1.id.cmp(&t2.id),
                SortItem::Done => t1.done.cmp(&t2.done),
            };

            match sort_direction {
                SortDirection::Asc => ordering,
                SortDirection::Desc => ordering.reverse(),
            }
        });
    }

    filtered
}

pub fn complete_task(tasks: &mut [Task], id: &u32) {
    match tasks.iter_mut().find(|t| t.id == *id) {
        Some(task) => task.done = true,
        None => eprintln!("Could not find task with id {}", id),
    };
}

pub fn print_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        let mark: &str = if task.done { "x" } else { " " };
        println!("{} [{}] {}", task.id, mark, task.description)
    }
}
