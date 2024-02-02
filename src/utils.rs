use crate::{
    Error::{AlreadyExists, NotASymlink, SdmwError},
    Result,
};
use filey::Filey;
use std::{env, fmt::Display, path::Path};

/// Returns the file name.
pub fn file_name<P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(Filey::new(path)
        .absolutized()
        .map_err(|e| e.into())
        .map_err(SdmwError)?
        .file_name()
        .unwrap())
}

pub fn print_already_exists<D: Display>(path: D) {
    eprintln!(
        "{}",
        AlreadyExists {
            path: path.to_string()
        }
    );
    eprintln!("Skipped");
}

pub fn print_not_a_symlink<D: Display>(path: D) {
    eprintln!(
        "error: {}",
        NotASymlink {
            path: path.to_string()
        }
    );
    eprintln!("Skipped");
}

/// Returns the current directory.
pub fn current_dir() -> Result<String> {
    Ok(env::current_dir()
        .map_err(|e| e.into())
        .map_err(SdmwError)?
        .to_string_lossy()
        .to_string())
}

/// Returns the absolute path.
pub fn absolutize<P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(Filey::new(path)
        .absolutized()
        .map_err(|e| e.into())
        .map_err(SdmwError)?
        .to_string())
}

/// Removes a file or a directory.
pub fn remove<P: AsRef<Path>>(path: P) -> filey::Result<()> {
    Filey::new(path).absolutized()?.remove()
}

/// Renames a file or a directory.
pub fn rename<P: AsRef<Path>>(from: P, to: P) -> filey::Result<()> {
    Filey::new(from)
        .absolutized()?
        .move_to(Filey::new(to).absolutized()?)?;
    Ok(())
}
