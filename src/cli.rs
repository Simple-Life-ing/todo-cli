use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(about = "A simple CLI todo app", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        title: String,
    },
    List {
        #[arg(short, long)]
        all: bool,
    },
    Done {
        id: usize,
    },
    Delete {
        id: usize,
    },
    Clear,
    Reset,
    Batch {
        titles: Vec<String>,
    },
    Search {
        keyword: String,
    },
    Export {
        path: String,
    },
    Import {
        path: String,

        #[arg(long)]
        preserve_id: bool,
    },
}
