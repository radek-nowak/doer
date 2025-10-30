use clap::{Parser, Subcommand, arg};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    List {
        #[arg(short, long)]
        all: bool,

        #[arg(short, long)]
        done: bool,

        #[arg(short, long, value_enum)]
        sort_by: Option<SortItem>,

        #[arg(long, value_enum, default_value_t=SortDirection::Asc)]
        sort_direction: SortDirection,
    },
    Complete {
        id: u32,
    },
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum SortItem {
    Id,
    Done,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

fn main() {
    let mut tasks = vec![
        Task {
            id: 1,
            description: String::from("First task"),
            done: true,
        },
        Task {
            id: 2,
            description: String::from("Second task"),
            done: false,
        },
        Task {
            id: 3,
            description: String::from("Third task"),
            done: false,
        },
    ];

    let cli = Cli::parse();

    match &cli.command {
        Commands::List {
            all,
            done,
            sort_by: sort_item,
            sort_direction,
        } => {
            let filtered = list_tasks(&tasks, *all, *done, sort_item, sort_direction);
            print_tasks(&filtered);
        }

        Commands::Complete { id } => {
            complete_task(&mut tasks, id);
            print_tasks(&tasks);
        }
    }
}

// TODO: remove reference from optionsal
fn list_tasks(
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

fn complete_task(tasks: &mut [Task], id: &u32) {
    match tasks.iter_mut().find(|t| t.id == *id) {
        Some(task) => task.done = true,
        None => eprintln!("Could not find task with id {}", id),
    };
}

fn print_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        let mark: &str = if task.done { "x" } else { " " };
        println!("{} [{}] {}", task.id, mark, task.description)
    }
}
