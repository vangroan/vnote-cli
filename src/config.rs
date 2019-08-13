//! Management of the user level config

use crate::errors::*;

/// Application config model
///
/// TODO: Load config from file
#[allow(dead_code)]
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
