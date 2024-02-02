use crate::{utils::*, Result, Settings};
use colored::Colorize;
use std::{fmt::Display, path::Path};

pub fn add(settings: &Settings, quiet: bool, pretend: bool) -> Result<()> {
    for path in settings.paths() {
        let file_name = &file_name(path)?;

        if !pretend && Path::new(file_name).exists() {
            continue;
        }

        if pretend {
            print_log(path, file_name)?;
            continue;
        }

        if let Err(e) = rename(path, &current_dir()?) {
            eprintln!("error: {}", e);
            continue;
        }

        if !quiet {
            print_log(path, file_name)?;
        }
    }
    Ok(())
}

fn print_log<D: Display>(from: D, to: D) -> Result<()> {
    eprintln!(
        "{} '{}' -> '{}/{}'",
        "Add".green().bold(),
        from,
        current_dir()?,
        to
    );
    Ok(())
}
