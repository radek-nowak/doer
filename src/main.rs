mod cli;
mod task;

use clap::Parser;
use cli::{Cli, Commands};
use task::{Task, complete_task, list_tasks, print_tasks};

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
