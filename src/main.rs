mod add;
mod install;
mod new;
mod parse_args;
mod settings;
mod test;
mod uninstall;
mod utils;

pub use crate::{add::add, install::install, new::new, settings::Settings, uninstall::uninstall};

use crate::parse_args::parse_args;
use std::process::exit;

fn main() {
    if let Err(e) = parse_args() {
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
