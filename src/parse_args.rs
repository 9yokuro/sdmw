use crate::{add, install, new, uninstall, Result, Settings};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"), arg_required_else_help = true, verbatim_doc_comment)]
struct Args {
    #[clap(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    /// Add files to a repository.
    Add,
    /// Create new repository.
    New { path: Vec<String> },
    /// Create symbolic links.
    Install,
    /// Delete symbolic links.
    Uninstall,
}

pub fn parse_args() -> Result<()> {
    let args = Args::parse();
    match args.subcommand {
        Subcommands::Add => add(&Settings::read("settings.json")?)?,
        Subcommands::New { path } => new(&path)?,
        Subcommands::Install => install(&Settings::read("settings.json")?)?,
        Subcommands::Uninstall => uninstall(&Settings::read("settings.json")?)?,
    }
    Ok(())
}
