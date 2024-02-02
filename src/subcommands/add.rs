use crate::{utils::*, Result, Settings};
use colored::Colorize;
use std::{fmt::Display, path::Path};

pub fn add(settings: &Settings, quiet: bool, pretend: bool) -> Result<()> {
    for i in settings.path() {
        let file_name = &file_name(i)?;

        if !pretend && Path::new(file_name).exists() {
            continue;
        }

        if pretend {
            show_success_message(i, file_name)?;
            continue;
        }

        if let Err(e) = mv(i, &current_dir()?) {
            eprintln!("error: {}", e);
            continue;
        }

        if !quiet {
            show_success_message(i, file_name)?;
        }
    }
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
