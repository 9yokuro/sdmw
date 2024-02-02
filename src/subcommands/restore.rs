use crate::{utils::*, Result, Settings, SETTINGS};
use colored::Colorize;
use std::fmt::Display;

pub fn restore(
    settings: &mut Settings,
    paths: &Vec<String>,
    quiet: bool,
    pretend: bool,
) -> Result<()> {
    for path in paths {
        for path_in_settings in settings.clone().paths() {
            let file_name = &file_name(path_in_settings)?;

            if absolutize(path)? == absolutize(file_name)? {
                if pretend {
                    print_log(file_name, path_in_settings);
                    continue;
                }

                if let Err(e) = rename(file_name, path_in_settings) {
                    eprintln!("error: {e}");
                    continue;
                }

                settings.remove(path_in_settings).write(SETTINGS)?;

                if !quiet {
                    print_log(file_name, path_in_settings);
                }
            }
        }
    }
    Ok(())
}

fn print_log<D: Display>(from: D, to: D) {
    eprintln!("{} '{}' -> '{}'", "Restored".green().bold(), from, to);
}
