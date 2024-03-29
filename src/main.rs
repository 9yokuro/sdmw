mod parse_arguments;
mod settings;
mod subcommands;
#[cfg(test)]
mod test;
mod utils;

pub use crate::{
    parse_arguments::Options,
    settings::{Settings, SETTINGS},
};

use crate::parse_arguments::parse_arguments;
use std::process::exit;

fn main() {
    if let Err(e) = parse_arguments() {
        eprintln!("error: {}", e);
        exit(1);
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    SdmwError(anyhow::Error),
    #[error("'{}' already exists", path)]
    AlreadyExists {
        path: String,
    },
    #[error("'{}' is not a symlink", path)]
    NotASymlink {
        path: String,
    },
    #[error("'{}' no such file or directory", path)]
    NotFound {
        path: String,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
