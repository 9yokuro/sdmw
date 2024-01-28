use crate::{utils::*, Result, Settings};
use colored::Colorize;
use std::{fmt::Display, path::Path};

pub fn uninstall(settings: &Settings) -> Result<()> {
    for i in settings.path() {
        let symlink = &absolutize(i)?;
        if !Path::new(symlink).is_symlink() {
            show_skip_deleting_symlink_message(symlink);
            continue;
        }

        if let Err(e) = delete_symlink(symlink) {
            eprintln!("error: {}", e);
            continue;
        }
        show_success_message(symlink);
    }
    Ok(())
}

fn show_success_message<D: Display>(path: D) {
    eprintln!("{} symlink '{}'", "Deleted".green().bold(), path);
}
