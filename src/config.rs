//! Management of the user level config

use crate::defaults::{DEFAULT_BOOK_NAME, DEFAULT_CONFIG_NAME, DEFAULT_DIR_NAME};
use serde::{Deserialize, Serialize};
use std::error;
use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application config model
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Notebook to use when no explicit notebook
    /// is defined in the command-line.
    default_book: String,
}

/// Interface of a backing storage of a config
pub trait ConfigStore {
    /// Ensures the underlying storage exists and is ready to use
    fn setup(&self) -> Result<()>;
    fn load_config(&self) -> Result<Config>;
}

pub struct ConfigFileStore {
    dir_path: PathBuf,
    file_name: &'static str,
}

impl ConfigFileStore {
    pub fn new(dir_path: &str, file_name: &'static str) -> ConfigFileStore {
        ConfigFileStore {
            dir_path: PathBuf::from(dir_path),
            file_name,
        }
    }
}

impl ConfigStore for ConfigFileStore {
    fn setup(&self) -> Result<()> {
        fs::create_dir_all(&self.dir_path)?;

        Ok(())
    }

    fn load_config(&self) -> Result<Config> {
        let mut path = self.dir_path.clone();
        path.push(self.file_name);

        if path.exists() {
            let file = File::open(path)?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;

            let config: Config = serde_yaml::from_str(&contents)?;

            Ok(config)
        } else {
            // Create config if not exist
            let config = Config {
                default_book: DEFAULT_BOOK_NAME.to_owned(),
            };

            let s = serde_yaml::to_string(&config)?;

            let mut file = File::create(path)?;
            file.write_all(s.as_bytes())?;

            Ok(config)
        }
    }
}

impl Default for ConfigFileStore {
    fn default() -> Self {
        let mut dir_path = dirs::home_dir().expect("Failed to determine your home directory");
        dir_path.push(DEFAULT_DIR_NAME);

        ConfigFileStore::new(
            dir_path.to_str().expect("Failed to create directory path"),
            DEFAULT_CONFIG_NAME,
        )
    }
}
