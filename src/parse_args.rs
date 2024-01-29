use crate::{add, install, new, uninstall, Result, Settings, SETTINGS};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"), arg_required_else_help = true, verbatim_doc_comment)]
struct Args {
    #[clap(subcommand)]
    subcommand: Subcommands,
    /// Do not print log messages.
    #[clap(short, long)]
    quiet: bool,
    /// Print what it would do but not actually change anything.
    #[clap(short, long)]
    pretend: bool,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    /// Add files to a repository.
    Add,
    /// Create symbolic links.
    Install,
    /// Create new repository.
    New { path: Vec<String> },
    /// Delete symbolic links.
    Uninstall,
}

pub fn parse_args() -> Result<()> {
    let args = Args::parse();

    let settings = &Settings::read(SETTINGS)?;

    match args.subcommand {
        Subcommands::Add => add(settings, args.quiet, args.pretend)?,
        Subcommands::Install => install(settings, args.quiet, args.pretend)?,
        Subcommands::New { path } => new(&path, args.quiet, args.pretend)?,
        Subcommands::Uninstall => uninstall(settings, args.quiet, args.pretend)?,
    }
    Ok(())
}
