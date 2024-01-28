use crate::{utils::*, Result, Settings};
use colored::Colorize;
use std::{
    ffi::OsStr,
    fmt::Display,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
    process::{Command, Stdio},
};

pub fn new(path: &Vec<String>, quiet: bool, pretend: bool) -> Result<()> {
    for i in path {
        if !pretend && Path::new(i).exists() {
            show_already_exists_message(i);
            continue;
        }

        if pretend {
            show_success_message_git(i);
        } else if let Err(e) = create_git_repository(i) {
            eprintln!("error: {}", e);
        } else if !quiet {
            show_success_message_git(i);
        }

        if pretend {
            show_success_message_file(format!("{}/settings.json", i));
        } else if let Err(e) = create_settings(i) {
            eprintln!("error: {}", e);
        } else if !quiet {
            show_success_message_file(format!("{}/settings.json", i));
        }

        if pretend {
            show_success_message_file(format!("{}/README.md", i));
        } else if let Err(e) = create_readme(i) {
            eprintln!("error: {}", e);
        } else if !quiet {
            show_success_message_file(format!("{}/README.md", i));
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

fn show_success_message_git<D: Display>(path: D) {
    eprintln!("{} repository '{}'", "Created".green().bold(), path);
}

fn create_settings<P: AsRef<Path>>(path: P) -> Result<()> {
    Settings::new(vec![]).write(format!("{}/settings.json", path.as_ref().to_string_lossy()))
}

fn create_readme<P: AsRef<Path>>(path: P) -> io::Result<()> {
    BufWriter::new(File::create(format!(
        "{}/README.md",
        path.as_ref().to_string_lossy()
    ))?)
    .write_all(b"# dotfiles")?;
    Ok(())
}

fn show_success_message_file<D: Display>(path: D) {
    eprintln!("{} {}", "Created".green().bold(), path);
}
