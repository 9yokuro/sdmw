use crate::{
    Error::{AlreadyExists, NotASymlink, SdmwError},
    Result,
};
use filey::Filey;
use std::{env, fmt::Display, path::Path};

pub fn file_name<P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(Filey::new(path)
        .absolutized()
        .map_err(|e| e.into())
        .map_err(SdmwError)?
        .file_name()
        .unwrap())
}

pub fn show_already_exists_message<D: Display>(path: D) {
    eprintln!(
        "{}",
        AlreadyExists {
            path: path.to_string()
        }
    );
    eprintln!("Skipped");
}

pub fn show_skip_deleting_symlink_message<D: Display>(path: D) {
    eprintln!(
        "error: {}",
        NotASymlink {
            path: path.to_string()
        }
    );
    eprintln!("Skipped");
}

pub fn current_dir() -> Result<String> {
    Ok(env::current_dir()
        .map_err(|e| e.into())
        .map_err(SdmwError)?
        .to_string_lossy()
        .to_string())
}

pub fn absolutize<P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(Filey::new(path)
        .absolutized()
        .map_err(|e| e.into())
        .map_err(SdmwError)?
        .to_string())
}

pub fn delete_symlink<P: AsRef<Path>>(path: P) -> filey::Result<()> {
    Filey::new(path).remove()
}
