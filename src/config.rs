//! Management of the user level config

use crate::defaults::{DEFAULT_BOOK_NAME, DEFAULT_CONFIG_NAME, DEFAULT_DIR_NAME};
use std::error;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application config model
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
    file_path: PathBuf,
}

impl ConfigFileStore {
    pub fn new(file_path: &str) -> ConfigFileStore {
        ConfigFileStore {
            file_path: PathBuf::from(file_path),
        }
    }
}

impl ConfigStore for ConfigFileStore {
    fn setup(&self) -> Result<()> {
        unimplemented!()
    }

    fn load_config(&self) -> Result<Config> {
        Ok(Config {
            default_book: DEFAULT_BOOK_NAME.to_owned(),
        })
    }
}

impl Default for ConfigFileStore {
    fn default() -> Self {
        let mut file_path = dirs::home_dir().expect("Failed to determine your home directory");
        file_path.push(DEFAULT_DIR_NAME);
        file_path.push(DEFAULT_CONFIG_NAME);

        ConfigFileStore::new(file_path.to_str().expect("Failed to create directory path"))
    }
}
