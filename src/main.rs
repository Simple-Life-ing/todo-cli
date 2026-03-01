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
        Commands::List { all } => service::list(all)?,
        Commands::Done { id } => service::done(id)?,
        Commands::Delete { id } => service::delete(id)?,
        Commands::Clear => service::clear()?,
        Commands::Reset => service::reset()?,
        Commands::Batch { titles } => service::batch_add(titles)?,
    }

    Ok(())
}
