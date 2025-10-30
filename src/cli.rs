use clap::{Parser, Subcommand, ValueEnum, arg};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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

#[derive(Debug, Clone, ValueEnum)]
pub enum SortItem {
    Id,
    Done,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SortDirection {
    Asc,
    Desc,
}
