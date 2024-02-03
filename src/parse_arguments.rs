use crate::{subcommands, Result, Settings, SETTINGS};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
    verbatim_doc_comment
)]
struct Arguments {
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
    /// Restore files.
    Restore { paths: Vec<String> },
    /// Create new repository.
    New { paths: Vec<String> },
    /// Delete symbolic links.
    Uninstall,
}

/// Command line options.
#[derive(Debug)]
pub struct Options {
    quiet: bool,
    pretend: bool,
}

impl Options {
    /// Constructs new Options.
    pub const fn new(quiet: bool, pretend: bool) -> Self {
        Self { quiet, pretend }
    }

    /// Returns quiet.
    pub const fn quiet(&self) -> bool {
        self.quiet
    }

    /// Returns pretend.
    pub const fn pretend(&self) -> bool {
        self.pretend
    }
}

pub fn parse_arguments() -> Result<()> {
    let arguments = Arguments::parse();

    let options = Options::new(arguments.quiet, arguments.pretend);

    match arguments.subcommand {
        Subcommands::Add => {
            let settings = Settings::read(SETTINGS)?;
            subcommands::add(&settings, &options)?
        }
        Subcommands::Install => {
            let settings = Settings::read(SETTINGS)?;
            subcommands::install(&settings, &options)?
        }
        Subcommands::New { paths } => subcommands::new(&paths, &options)?,
        Subcommands::Restore { paths } => {
            let mut settings = Settings::read(SETTINGS)?;
            subcommands::restore(&mut settings, &paths, &options)?
        }
        Subcommands::Uninstall => {
            let settings = Settings::read(SETTINGS)?;
            subcommands::uninstall(&settings, &options)?
        }
    }
    Ok(())
}
