use crate::{utils::*, Result, Settings};
use colored::Colorize;
use filey::{self, Error::FileyError, Filey};
use std::{fmt::Display, path::Path};

pub fn add(settings: &Settings) -> Result<()> {
    for i in settings.path() {
        let file_name = &file_name(i)?;
        if Path::new(file_name).exists() {
            continue;
        }

        if let Err(e) = move_file(i) {
            eprintln!("error: {}", e);
            continue;
        }
        show_success_message(i, file_name)?;
    }
    Ok(())
}

fn move_file<P: AsRef<Path>>(path: P) -> filey::Result<()> {
    Filey::new(path)
        .absolutized()?
        .move_to(current_dir().map_err(|e| e.into()).map_err(FileyError)?)?;
    Ok(())
}

fn show_success_message<D: Display>(original: D, file_name: D) -> Result<()> {
    eprintln!(
        "{} '{}' -> '{}/{}'",
        "Add".green().bold(),
        original,
        current_dir()?,
        file_name
    );
    Ok(())
}
