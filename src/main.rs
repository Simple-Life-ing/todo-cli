mod cli;
mod model;
mod service;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title } => service::add(title)?,
        Commands::List => service::list()?,
        _ => println!("功能开发中"),
    }

    Ok(())
}
