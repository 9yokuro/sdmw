use crate::{utils::*, Error::NotFound, Options, Result, Settings};
use colored::Colorize;
use filey::{self, Filey};
use std::{fmt::Display, path::Path};

pub fn install(settings: &Settings, options: &Options) -> Result<()> {
    for path in settings.paths() {
        let symlink = &absolutize(path)?;
        let original = &file_name(symlink)?;

        if !options.pretend() && !Path::new(&absolutize(original)?).exists() {
            eprintln!(
                "error: {}",
                NotFound {
                    path: original.to_string()
                }
            );
            continue;
        }

        if options.pretend() {
            print_log(original, symlink);
            continue;
        }

        if let Err(e) = create_symlink(original, symlink) {
            eprintln!("error: {}", e);
            continue;
        }

        if !options.quiet() {
            print_log(original, symlink);
        }
    }
    Ok(())
}

fn create_symlink<P: AsRef<Path>>(original: P, symlink: P) -> filey::Result<()> {
    Filey::new(original)
        .absolutize()?
        .symlink(Filey::new(symlink).absolutize()?)
}

fn print_log<D: Display>(original: D, symlink: D) {
    eprintln!(
        "{} symlink '{}' -> '{}'",
        "Created".green().bold(),
        original,
        symlink
    );
}
