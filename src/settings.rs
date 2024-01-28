use crate::{Error::SdmwError, Result};
use filey::Filey;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

/// Setting
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Settings {
    path: Vec<String>,
}

impl Settings {
    /// Constructs new Settings
    pub fn new(path: Vec<String>) -> Self {
        Self { path }
    }

    /// Returns path.
    pub fn path(&self) -> &Vec<String> {
        &self.path
    }

    /// Reads a configuration file and Returns Settings.
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let sdmw =
            serde_json::from_reader(File::open(path).map_err(|e| e.into()).map_err(SdmwError)?)
                .map_err(|e| e.into())
                .map_err(SdmwError)?;
        Ok(sdmw)
    }

    /// Write Settings to a configuration file.
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        serde_json::to_writer_pretty(
            File::create(path)
                .map_err(|e| e.into())
                .map_err(SdmwError)?,
            &self,
        )
        .map_err(|e| e.into())
        .map_err(SdmwError)?;
        Ok(())
    }

    /// Remove a file from Settings.
    pub fn remove<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.path.retain(|p| {
            Filey::new(p).absolutized().unwrap() != Filey::new(path.as_ref()).absolutized().unwrap()
        });
        self
    }
}
