use crate::{utils::*, Error::NotFound, Result, Settings};
use colored::Colorize;
use filey::{self, Filey};
use std::{fmt::Display, path::Path};

pub fn install(settings: &Settings, quiet: bool, pretend: bool) -> Result<()> {
    for i in settings.path() {
        let symlink = &absolutize(i)?;
        if !pretend && Path::new(symlink).exists() {
            show_already_exists_message(symlink);
            continue;
        }

        let original = file_name(symlink)?;
        if !pretend && !Path::new(&absolutize(&original)?).exists() {
            eprintln!(
                "error: {}",
                NotFound {
                    path: original.to_string()
                }
            );
            continue;
        }
        if pretend {
            show_success_message(&original, symlink);
            continue;
        }
        if let Err(e) = create_symlink(&original, symlink) {
            eprintln!("error: {}", e);
            continue;
        }
        if !quiet {
            show_success_message(&original, symlink);
        }
    }
    Ok(())
}

fn create_symlink<P: AsRef<Path>>(original: P, symlink: P) -> filey::Result<()> {
    Filey::new(original).symlink(Filey::new(symlink).absolutized()?)
}

fn show_success_message<D: Display>(original: D, symlink: D) {
    eprintln!(
        "{} symlink '{}' -> '{}'",
        "Created".green().bold(),
        original,
        symlink
    );
}
