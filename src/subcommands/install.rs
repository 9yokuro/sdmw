use crate::{utils::*, Error::NotFound, Result, Settings};
use colored::Colorize;
use filey::{self, Filey};
use std::{fmt::Display, path::Path};

pub fn install(settings: &Settings, quiet: bool, pretend: bool) -> Result<()> {
    for path in settings.paths() {
        let symlink = &absolutize(path)?;

        if !pretend && Path::new(symlink).exists() {
            print_already_exists(symlink);
            continue;
        }

        let original = &file_name(symlink)?;

        if !pretend && !Path::new(&absolutize(original)?).exists() {
            eprintln!(
                "error: {}",
                NotFound {
                    path: original.to_string()
                }
            );
            continue;
        }

        if pretend {
            print_log(original, symlink);
            continue;
        }

        if let Err(e) = create_symlink(original, symlink) {
            eprintln!("error: {}", e);
            continue;
        }

        if !quiet {
            print_log(original, symlink);
        }
    }
    Ok(())
}

fn create_symlink<P: AsRef<Path>>(original: P, symlink: P) -> filey::Result<()> {
    Filey::new(original)
        .absolutized()?
        .symlink(Filey::new(symlink).absolutized()?)
}

fn print_log<D: Display>(original: D, symlink: D) {
    eprintln!(
        "{} symlink '{}' -> '{}'",
        "Created".green().bold(),
        original,
        symlink
    );
}
