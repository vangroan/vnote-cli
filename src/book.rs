use chrono::{DateTime, Local};
use uuid::Uuid;

use std::collections::HashMap;
use std::error;
use std::fs;
use std::path::PathBuf;
use std::vec::Vec;

pub const DEFAULT_DIR_NAME : &str = ".vnote";
pub const DEFAULT_BOOK_NAME : &str = "vnote";

pub type Result<T> = std::result::Result<T, Box<error::Error>>;

/// Represents the contents of a notebook file.
#[allow(dead_code)]
pub struct Notebook(HashMap<String, Vec<String>>);

#[allow(dead_code)]
pub struct Note {
    id: Uuid,
    content: String,
    date: DateTime<Local>,
}

impl Note {
    /// Creates a new Note
    pub fn new(content: String) -> Note {
        Note {
            id: Uuid::new_v4(),
            content,
            date: Local::now(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

pub trait NotebookStore {
    /// Ensures the underlying storage exists and is ready to use
    fn setup(&self) -> Result<()>;
    fn add_note(&self, note: Note, book_name: Option<&str>);
}

pub struct NotebookFileStorage {
    dir_path: PathBuf,
    /// The path of the default book
    file_path: PathBuf,
}

impl NotebookFileStorage {
    /// Creates a new filestorage instance
    pub fn new(dir_path: &str, file_name: &str) -> NotebookFileStorage {
        let mut file_path = PathBuf::from(dir_path);
        file_path.push(file_name);

        // TODO: Log trace file path of default book and directory

        NotebookFileStorage {
            dir_path: PathBuf::from(dir_path),
            file_path,
        }
    }
}

impl NotebookStore for NotebookFileStorage {
    fn setup(&self) -> Result<()> {
        fs::create_dir_all(&self.dir_path)?;

        Ok(())
    }

    fn add_note(&self, _note: Note, _book_name: Option<&str>) {
        unimplemented!()
    }
}

impl Default for NotebookFileStorage {
    fn default() -> Self {
        let mut dir_path = std::env::home_dir().expect("Failed to determine your home directory");
        dir_path.push(DEFAULT_DIR_NAME);

        NotebookFileStorage::new(dir_path.to_str().expect("Failed to create directory path"), DEFAULT_BOOK_NAME)
    }
}
