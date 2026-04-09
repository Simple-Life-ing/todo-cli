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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_add_command() {
        let cli = Cli::parse_from(["todo", "add", "learn-ci"]);

        match cli.command {
            Commands::Add { title } => assert_eq!(title, "learn-ci"),
            other => panic!("expected add command, got {other:?}"),
        }
    }

    #[test]
    fn parses_list_all_flag() {
        let cli = Cli::parse_from(["todo", "list", "--all"]);

        match cli.command {
            Commands::List { all } => assert!(all),
            other => panic!("expected list command, got {other:?}"),
        }
    }

    #[test]
    fn parses_import_with_preserve_id() {
        let cli = Cli::parse_from(["todo", "import", "todos.json", "--preserve-id"]);

        match cli.command {
            Commands::Import { path, preserve_id } => {
                assert_eq!(path, "todos.json");
                assert!(preserve_id);
            }
            other => panic!("expected import command, got {other:?}"),
        }
    }
}
