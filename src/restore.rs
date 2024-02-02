use crate::{utils::*, Result, Settings, SETTINGS};
use colored::Colorize;
use std::fmt::Display;

pub fn restore(
    settings: &mut Settings,
    path: &Vec<String>,
    quiet: bool,
    pretend: bool,
) -> Result<()> {
    for p in path {
        for q in settings.clone().path() {
            let file_name = &file_name(q)?;

            if absolutize(p)? == absolutize(file_name)? {
                if pretend {
                    show_success_message(file_name, q);
                    continue;
                }

                if let Err(e) = mv(file_name, q) {
                    eprintln!("error: {e}");
                    continue;
                }

                settings.remove(q).write(SETTINGS)?;

                if !quiet {
                    show_success_message(file_name, q);
                }
            }
        }
    }
    Ok(())
}

fn show_success_message<D: Display>(from: D, to: D) {
    eprintln!("{} '{}' -> '{}'", "Restored".green().bold(), from, to);
}
