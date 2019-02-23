use chrono::{DateTime, Local};
use uuid::Uuid;

use std::collections::HashMap;
use std::vec::Vec;

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
