use crate::{utils::*, Options, Result, Settings};
use colored::Colorize;
use std::{fmt::Display, path::Path};

pub fn add(settings: &Settings, options: &Options) -> Result<()> {
    for path in settings.paths() {
        let file_name = &file_name(path)?;

        if !options.pretend() && Path::new(file_name).exists() {
            continue;
        }

        if options.pretend() {
            print_log(path, file_name)?;
            continue;
        }

        if let Err(e) = rename(path, &current_dir()?) {
            eprintln!("error: {}", e);
            continue;
        }

        if !options.quiet() {
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
