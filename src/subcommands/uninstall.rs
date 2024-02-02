use crate::{utils::*, Result, Settings};
use colored::Colorize;
use std::{fmt::Display, path::Path};

pub fn uninstall(settings: &Settings, quiet: bool, pretend: bool) -> Result<()> {
    for path in settings.paths() {
        let symlink = &absolutize(path)?;

        if !pretend && !Path::new(symlink).is_symlink() {
            print_not_a_symlink(symlink);
            continue;
        }

        if pretend {
            print_log(symlink);
            continue;
        }

        if let Err(e) = remove(symlink) {
            eprintln!("error: {}", e);
            continue;
        }

        if !quiet {
            print_log(symlink);
        }
    }
    Ok(())
}

fn print_log<D: Display>(path: D) {
    eprintln!("{} symlink '{}'", "Deleted".green().bold(), path);
}
