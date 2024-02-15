use crate::{
    pretend, print_log, rename, utils::*, Error::SdmwError, Options, Result, Settings, SETTINGS,
};
use colored::Colorize;
use filey::Filey;
use std::{fmt::Display, path::Path};

pub fn add(settings: &mut Settings, paths: Option<&Vec<String>>, options: &Options) -> Result<()> {
    add_statically(&settings.clone(), options)?;

    if let Some(paths) = paths {
        add_dynamically(settings, paths, options)?;
    }
    Ok(())
}

fn add_statically(settings: &Settings, options: &Options) -> Result<()> {
    for path in settings.paths() {
        let file_name = &file_name(path)?;

        if !options.pretend() && Path::new(file_name).exists() {
            continue;
        }

        pretend!(options, path, file_name);

        rename!(path, &current_dir()?);

        print_log!(options, path, file_name);
    }
    Ok(())
}

fn add_dynamically(settings: &mut Settings, paths: &Vec<String>, options: &Options) -> Result<()> {
    for path in paths {
        let file_name = &file_name(path)?;

        pretend!(options, path, file_name);

        rename!(path, &current_dir()?);

        settings.add(format_path(path)?).write(SETTINGS)?;

        print_log!(options, path, file_name);
    }
    Ok(())
}

#[macro_export]
macro_rules! pretend {
    ($options:expr, $from:expr, $to:expr) => {
        if $options.pretend() {
            print_log($from, $to)?;
            continue;
        }
    };
}

#[macro_export]
macro_rules! print_log {
    ($options:expr, $from:expr, $to:expr) => {
        if !$options.quiet() {
            print_log($from, $to)?;
        }
    };
}

#[macro_export]
macro_rules! rename {
    ($from:expr, $to:expr) => {
        if let Err(e) = rename($from, $to) {
            eprintln!("error: {}", e);
            continue;
        }
    };
}

fn format_path<P: AsRef<Path>>(path: P) -> Result<String> {
    let filey = Filey::new(path)
        .absolutize()
        .map_err(|e| e.into())
        .map_err(SdmwError)?
        .contract_user()
        .map_err(|e| e.into())
        .map_err(SdmwError)?
        .to_string();
    Ok(filey)
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
