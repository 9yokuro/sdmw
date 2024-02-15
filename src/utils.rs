use crate::{
    Error::{AlreadyExists, NotASymlink, SdmwError},
    Result,
};
use filey::Filey;
use std::{env, fmt::Display, path::Path};

/// Returns the file name.
pub fn file_name<P: AsRef<Path>>(path: P) -> Result<String> {
    let file_name = Filey::new(path)
        .absolutize()
        .map_err(|e| e.into())
        .map_err(SdmwError)?
        .file_name()
        .unwrap();
    Ok(file_name)
}

pub fn print_already_exists<D: Display>(path: D) {
    eprintln!(
        "error: {}",
        AlreadyExists {
            path: path.to_string()
        }
    );
}

pub fn print_not_a_symlink<D: Display>(path: D) {
    eprintln!(
        "error: {}",
        NotASymlink {
            path: path.to_string()
        }
    );
}

/// Returns the current directory.
pub fn current_dir() -> Result<String> {
    let current_dir = env::current_dir()
        .map_err(|e| e.into())
        .map_err(SdmwError)?;
    let current_dir_string = asref_path_to_string(current_dir);
    Ok(current_dir_string)
}

/// Returns the absolute path.
pub fn absolutize<P: AsRef<Path>>(path: P) -> Result<String> {
    let absolutized = Filey::new(path)
        .expand_user()
        .map_err(|e| e.into())
        .map_err(SdmwError)?
        .absolutize()
        .map_err(|e| e.into())
        .map_err(SdmwError)?
        .to_string();
    Ok(absolutized)
}

/// Removes a file or a directory.
pub fn remove<P: AsRef<Path>>(path: P) -> Result<()> {
    Filey::new(path)
        .absolutize()
        .map_err(|e| e.into())
        .map_err(SdmwError)?
        .remove()
        .map_err(|e| e.into())
        .map_err(SdmwError)
}

/// Renames a file or a directory.
pub fn rename<P: AsRef<Path>>(from: P, to: P) -> Result<()> {
    let path = absolutize(to)?;
    Filey::new(from)
        .absolutize()
        .map_err(|e| e.into())
        .map_err(SdmwError)?
        .move_to(path)
        .map_err(|e| e.into())
        .map_err(SdmwError)?;
    Ok(())
}

pub fn asref_path_to_string<P: AsRef<Path>>(path: P) -> String {
    path.as_ref().to_string_lossy().to_string()
}
