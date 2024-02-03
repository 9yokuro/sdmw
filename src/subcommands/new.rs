use crate::{utils::*, Options, Result, Settings, SETTINGS};
use colored::Colorize;
use std::{
    ffi::OsStr,
    fmt::Display,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
    process::{Command, Stdio},
};

const README: &str = "README.md";

pub fn new(paths: &Vec<String>, options: &Options) -> Result<()> {
    for path in paths {
        if !options.pretend() && Path::new(path).exists() {
            print_already_exists(path);
            continue;
        }

        let message = &format!("repository '{}'", path);
        if options.pretend() {
            print_log(message);
        } else if let Err(e) = create_git_repository(path) {
            eprintln!("error: {}", e);
        } else if !options.quiet() {
            print_log(message);
        }

        if options.pretend() {
            print_log(path_to_settings(path));
        } else if let Err(e) = create_settings(path) {
            eprintln!("error: {}", e);
        } else if !options.quiet() {
            print_log(path_to_settings(path));
        }

        if options.pretend() {
            print_log(path_to_readme(path));
        } else if let Err(e) = create_readme(path) {
            eprintln!("error: {}", e);
        } else if !options.quiet() {
            print_log(path_to_readme(path));
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

fn create_settings<P: AsRef<Path>>(path: P) -> Result<()> {
    Settings::new(vec![]).write(path_to_settings(path))
}

fn path_to_settings<P: AsRef<Path>>(path: P) -> String {
    let pathbuf = path.as_ref().join(SETTINGS);
    asref_path_to_string(pathbuf)
}

fn create_readme<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let readme = File::create(path_to_readme(path))?;
    BufWriter::new(readme).write_all(b"# dotfiles")?;
    Ok(())
}

fn path_to_readme<P: AsRef<Path>>(path: P) -> String {
    let pathbuf = path.as_ref().join(README);
    asref_path_to_string(pathbuf)
}

fn print_log<D: Display>(path: D) {
    eprintln!("{} {}", "Created".green().bold(), path);
}
