mod cli;
mod model;
mod service;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut conn = storage::get_connection()?;

    match cli.command {
        Commands::Add { title } => service::add(&conn, title)?,
        Commands::List { all } => service::list(&conn, all)?,
        Commands::Done { id } => service::done(&conn, id)?,
        Commands::Delete { id } => service::delete(&conn, id)?,
        Commands::Clear => service::clear(&mut conn)?,
        Commands::Reset => service::reset(&mut conn)?,
        Commands::Batch { titles } => service::batch_add(&mut conn, titles)?,
        Commands::Search { keyword } => service::search(&conn, keyword)?,
        Commands::Export { path } => service::export_json(&conn, path)?,
        Commands::Import { path, preserve_id } => {
            service::import_json(&mut conn, path, preserve_id)?
        }
    }

    Ok(())
}
