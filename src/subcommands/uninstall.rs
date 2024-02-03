use crate::{utils::*, Options, Result, Settings};
use colored::Colorize;
use std::{fmt::Display, path::Path};

pub fn uninstall(settings: &Settings, options: &Options) -> Result<()> {
    for path in settings.paths() {
        let symlink = &absolutize(path)?;

        if !options.pretend() && !Path::new(symlink).is_symlink() {
            print_not_a_symlink(symlink);
            continue;
        }

        if options.pretend() {
            print_log(symlink);
            continue;
        }

        if let Err(e) = remove(symlink) {
            eprintln!("error: {}", e);
            continue;
        }

        if !options.quiet() {
            print_log(symlink);
        }
    }
    Ok(())
}

fn print_log<D: Display>(path: D) {
    eprintln!("{} symlink '{}'", "Deleted".green().bold(), path);
}
