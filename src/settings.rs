use crate::{utils::*, Error::SdmwError, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

pub const SETTINGS: &str = "settings.json";

/// Setting
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Settings {
    paths: Vec<String>,
}

impl Settings {
    /// Constructs new Settings
    pub const fn new(paths: Vec<String>) -> Self {
        Self { paths }
    }

    /// Returns paths.
    pub const fn paths(&self) -> &Vec<String> {
        &self.paths
    }

    /// Reads a configuration file and Returns Settings.
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let json = File::open(path).map_err(|e| e.into()).map_err(SdmwError)?;
        let sdmw = serde_json::from_reader(json)
            .map_err(|e| e.into())
            .map_err(SdmwError)?;
        Ok(sdmw)
    }

    /// Writes Settings to a file.
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = File::create(path)
            .map_err(|e| e.into())
            .map_err(SdmwError)?;
        serde_json::to_writer_pretty(json, &self)
            .map_err(|e| e.into())
            .map_err(SdmwError)?;
        Ok(())
    }

    /// Removes an element.
    pub fn remove<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        let target = &asref_path_to_string(path);
        self.paths.retain(|p| p != target);
        self
    }

    /// Adds an element.
    pub fn add<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.paths.push(asref_path_to_string(path));
        self
    }
}
