use crate::{utils::*, Result, Settings, SETTINGS};
use colored::Colorize;
use std::{
    ffi::OsStr,
    fmt::Display,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
    process::{Command, Stdio},
};

pub fn new(paths: &Vec<String>, quiet: bool, pretend: bool) -> Result<()> {
    for path in paths {
        if !pretend && Path::new(path).exists() {
            print_already_exists(path);
            continue;
        }

        if pretend {
            print_git_log(path);
        } else if let Err(e) = create_git_repository(path) {
            eprintln!("error: {}", e);
        } else if !quiet {
            print_git_log(path);
        }

        if pretend {
            print_create_file_log(format!("{}/{}", path, SETTINGS));
        } else if let Err(e) = create_settings(path) {
            eprintln!("error: {}", e);
        } else if !quiet {
            print_create_file_log(format!("{}/{}", path, SETTINGS));
        }

        if pretend {
            print_create_file_log(format!("{}/README.md", path));
        } else if let Err(e) = create_readme(path) {
            eprintln!("error: {}", e);
        } else if !quiet {
            print_create_file_log(format!("{}/README.md", path));
        }
    }
    Ok(())
}

fn create_git_repository<O: AsRef<OsStr>>(path: O) -> io::Result<()> {
    Command::new("git")
        .arg("init")
        .arg("-b")
        .arg("main")
        .arg(path)
        .stdout(Stdio::null())
        .spawn()?
        .wait()?;
    Ok(())
}

fn print_git_log<D: Display>(path: D) {
    eprintln!("{} repository '{}'", "Created".green().bold(), path);
}

fn create_settings<P: AsRef<Path>>(path: P) -> Result<()> {
    Settings::new(vec![]).write(format!("{}/{}", path.as_ref().to_string_lossy(), SETTINGS))
}

fn create_readme<P: AsRef<Path>>(path: P) -> io::Result<()> {
    BufWriter::new(File::create(format!(
        "{}/README.md",
        path.as_ref().to_string_lossy()
    ))?)
    .write_all(b"# dotfiles")?;
    Ok(())
}

fn print_create_file_log<D: Display>(path: D) {
    eprintln!("{} {}", "Created".green().bold(), path);
}
