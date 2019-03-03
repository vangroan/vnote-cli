use chrono::{DateTime, Local};
use regex::RegexBuilder;
use serde::{Serialize, Deserialize};
use serde_yaml;
use uuid::Uuid;

use std::collections::HashMap;
use std::error;
use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::vec::Vec;

pub const DEFAULT_DIR_NAME : &str = ".vnote";
pub const DEFAULT_BOOK_NAME : &str = "vnote";

/// The threshold where the edit distance considers typos.
/// 
/// The value is inclusive.
#[allow(dead_code)]
pub const TYPO_DISTANCE : f64 = 0.7;

pub type Result<T> = std::result::Result<T, Box<error::Error>>;

/// Calculates how similar two strings are, returning a value
/// between 0.0 and 1.0.
/// 
/// * 0.0 means the two strings are nothing alike
/// * 0.5 means the two strings are half alike
/// * 1.0 means the two strings are identical
#[allow(dead_code)]
fn edit_distance(a: &str, b: &str) -> f64 {
    let len = std::cmp::max(a.len(), b.len()) as f64;
    ((len - levenshtein::levenshtein(a, b) as f64) / len)
}

/// Represents the contents of a notebook file.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Notebook(HashMap<String, Vec<Note>>);

impl Default for Notebook {
    fn default() -> Self {
        Notebook(HashMap::new())
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
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

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub trait NotebookStore {
    /// Ensures the underlying storage exists and is ready to use
    fn setup(&self) -> Result<()>;
    /// Adds a note to a book, and commit it to storage
    fn add_note(&self, topic: &str, note: Note, book_name: Option<&str>) -> Result<()>;
    fn load_book(&self, book_name: &str) -> Result<Notebook>;
    fn save_book(&self, book_name: &str, book: Notebook) -> Result<()>;
    // Searches an entire notebook for each note that matches the given pattern
    fn scan_notes(&self, pattern: &str, book_name: Option<&str>, topic_name: Option<&str>) -> Result<Vec<(String, Note)>>;
}

#[allow(dead_code)]
pub struct NotebookFileStorage {
    dir_path: PathBuf,
    /// The name of the default notebook
    book_name: String,
}

impl NotebookFileStorage {
    /// Creates a new filestorage instance
    pub fn new(dir_path: &str, book_name: &str) -> NotebookFileStorage {
        // TODO: Log trace file path of default book and directory

        NotebookFileStorage {
            dir_path: PathBuf::from(dir_path),
            book_name: book_name.to_string(),
        }
    }
}

impl NotebookStore for NotebookFileStorage {
    fn setup(&self) -> Result<()> {
        fs::create_dir_all(&self.dir_path)?;

        Ok(())
    }

    fn add_note(&self, topic: &str, note: Note, book_name: Option<&str>) -> Result<()> {
        let name = match book_name {
            Some(s) => s,
            None => &self.book_name,
        };

        let mut book = self.load_book(name)?;

        book.0.entry(topic.to_string())
            .or_insert(vec![])
            .push(note);
        
        self.save_book(name, book)?;

        Ok(())
    }

    fn load_book(&self, book_name: &str) -> Result<Notebook> {
        let mut path = self.dir_path.clone();
        path.push(book_name);

        if path.exists() {
            let file = File::open(path)?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;

            let book : Notebook = serde_yaml::from_str(&contents)?;

            Ok(book)
        } else {
            Ok(Notebook::default())
        }
    }

    fn save_book(&self, book_name: &str, book: Notebook) -> Result<()> {
        let mut path = self.dir_path.clone();
        path.push(book_name);

        let s = serde_yaml::to_string(&book)?;

        let mut file = File::create(path)?;
        file.write_all(s.as_bytes())?;

        Ok(())
    }

    fn scan_notes(&self, pattern: &str, book_name: Option<&str>, topic_name: Option<&str>) -> Result<Vec<(String, Note)>> {
        let re = RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build()?;
        let book = self.load_book(book_name.unwrap_or(DEFAULT_BOOK_NAME))?;

        // Keeping iterator options on stack to avoid Box when upcast to Iterator
        let mut filter_iter;
        let mut iter;
        
        // consume book, importantly don't save it back
        let iter : &mut Iterator<Item=(String, Vec<Note>)> = match topic_name {
            Some(t) => {
                filter_iter = book.0.into_iter().filter(move |(topic, _notes)| t == topic);
                &mut filter_iter
            }
            None => {
                iter = book.0.into_iter();
                &mut iter
            }
        };
        
        // NOTE: Copying strings. investigate more efficient solution
        Ok(iter
            .flat_map(|(topic, notes)| notes.into_iter().map(move |note| (topic.clone(), note)))
            .filter(|(_topic, note)| re.is_match(&note.content))
            .collect())
    }
}

impl Default for NotebookFileStorage {
    fn default() -> Self {
        let mut dir_path = dirs::home_dir().expect("Failed to determine your home directory");
        dir_path.push(DEFAULT_DIR_NAME);

        NotebookFileStorage::new(dir_path.to_str().expect("Failed to create directory path"), DEFAULT_BOOK_NAME)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_edit_distance() {
        assert_eq!(1.0, edit_distance("javascript", "javascript"));
        assert_eq!(0.5, edit_distance("javascript", "javasxxxxx"));
        assert_eq!(0.0, edit_distance("javascript", "xxxxxxxxxx"));
        assert_eq!(0.9, edit_distance("javascript", "javscript"));
        assert_eq!(0.8, edit_distance("javascript", "javscriptt"));
        assert_eq!(0.8, edit_distance("javascript", "jaavscript"));
    }
}