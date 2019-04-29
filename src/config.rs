//! Management of the user level config

use std::error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application config model
pub struct Config {
    /// Notebook to use when no excplicit notebook
    /// is defined in the command-line.
    default_book: String,
}

/// Interface of a backing storage of a config
pub trait ConfigStore {
    /// Ensures the underlying storage exists and is ready to use
    fn setup(&self) -> Result<()>;
    fn load(&self) -> Result<Config>;
}
