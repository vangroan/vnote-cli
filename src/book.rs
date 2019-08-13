use crate::errors;
use chrono::{DateTime, Local};
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use serde_yaml;
use uuid::Uuid;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::vec::Vec;

pub const DEFAULT_DIR_NAME: &str = ".vnote";
pub const DEFAULT_BOOK_NAME: &str = "vnote";

/// The threshold where the edit distance considers typos.
///
/// The value is inclusive.
pub const TYPO_DISTANCE: f64 = 0.7;

/// Calculates how similar two strings are, returning a value
/// between 0.0 and 1.0.
///
/// * 0.0 means the two strings are nothing alike
/// * 0.5 means the two strings are half alike
/// * 1.0 means the two strings are identical
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
    fn setup(&self) -> errors::Result<()>;
    /// Adds a note to a book, and commit it to storage
    fn add_note(&self, topic: &str, note: Note, book_name: Option<&str>) -> errors::Result<()>;
    fn load_book(&self, book_name: &str) -> errors::Result<Notebook>;
    fn save_book(&self, book_name: &str, book: Notebook) -> errors::Result<()>;
    // Searches an entire notebook for each note that matches the given pattern
    #[deprecated(since = "1.1", note = "searching has moved to NotebookSearch")]
    fn scan_notes(
        &self,
        pattern: &str,
        book_name: Option<&str>,
        topic_name: Option<&str>,
    ) -> errors::Result<Vec<(String, Note)>>;
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
    fn setup(&self) -> errors::Result<()> {
        fs::create_dir_all(&self.dir_path)?;

        Ok(())
    }

    fn add_note(&self, topic: &str, note: Note, book_name: Option<&str>) -> errors::Result<()> {
        let name = match book_name {
            Some(s) => s,
            None => &self.book_name,
        };

        let mut book = self.load_book(name)?;

        book.0.entry(topic.to_string()).or_insert(vec![]).push(note);

        self.save_book(name, book)?;

        Ok(())
    }

    fn load_book(&self, book_name: &str) -> errors::Result<Notebook> {
        let mut path = self.dir_path.clone();
        path.push(book_name);

        if path.exists() {
            let file = File::open(path)?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;

            let book: Notebook = serde_yaml::from_str(&contents)?;

            Ok(book)
        } else {
            Ok(Notebook::default())
        }
    }

    fn save_book(&self, book_name: &str, book: Notebook) -> errors::Result<()> {
        let mut path = self.dir_path.clone();
        path.push(book_name);

        let s = serde_yaml::to_string(&book)?;

        let mut file = File::create(path)?;
        file.write_all(s.as_bytes())?;

        Ok(())
    }

    fn scan_notes(
        &self,
        pattern: &str,
        book_name: Option<&str>,
        topic_name: Option<&str>,
    ) -> errors::Result<Vec<(String, Note)>> {
        let re = RegexBuilder::new(pattern).case_insensitive(true).build()?;
        let book = self.load_book(book_name.unwrap_or(DEFAULT_BOOK_NAME))?;

        // Keeping iterator options on stack to avoid Box when upcast to Iterator
        let mut filter_iter;
        let mut iter;

        // consume book, importantly don't save it back
        let iter: &mut Iterator<Item = (String, Vec<Note>)> = match topic_name {
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

        NotebookFileStorage::new(
            dir_path.to_str().expect("Failed to create directory path"),
            DEFAULT_BOOK_NAME,
        )
    }
}

pub struct NotebookSearch;

impl NotebookSearch {
    pub fn new() -> NotebookSearch {
        NotebookSearch
    }

    /// Scans a notebook to find an exact or close match of a topic name
    pub fn match_topic<'a>(&self, topic: &str, book: &'a Notebook) -> PossibleTopic<'a> {
        let mut filtered: Vec<(&'a str, f64)> = book
            .0
            .iter()
            .map(|(t, _)| {
                let distance = edit_distance(topic, t);
                (t.as_str(), distance)
            })
            .filter(|tuple| tuple.1 >= TYPO_DISTANCE)
            .collect();

        // Put the closest match first
        // TODO: instead of sorting, we can just iterate and keep track of the highest distance and index
        filtered.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let top_result = filtered.into_iter().take(1).next();

        match top_result {
            Some((t, distance)) => {
                if distance == 1.0 {
                    PossibleTopic::Exact
                } else {
                    PossibleTopic::CloseMatch { topic: t, distance }
                }
            }
            None => PossibleTopic::Nothing,
        }
    }

    pub fn scan_notes<'a>(
        &self,
        pattern: &str,
        topic_name: Option<&str>,
        book: &'a Notebook,
    ) -> errors::Result<SearchResults<'a>> {
        let re = RegexBuilder::new(pattern).case_insensitive(true).build()?;

        // Keeping iterator options on stack to avoid Box when upcast to Iterator
        let mut filter_iter;
        let mut iter;

        // consume book, importantly don't save it back
        let iter: &mut Iterator<Item = (&String, &Vec<Note>)> = match topic_name {
            Some(t) => {
                filter_iter = book
                    .0
                    .iter()
                    .filter(move |(topic, _notes)| t == topic.as_str());
                &mut filter_iter
            }
            None => {
                iter = book.0.iter();
                &mut iter
            }
        };

        // NOTE: Copying strings. investigate more efficient solution
        Ok(SearchResults(
            iter.flat_map(|(topic, notes)| notes.into_iter().map(move |note| (topic, note)))
                .filter(|(_topic, note)| re.is_match(&note.content))
                .map(move |(topic, note)| (topic.as_str(), note))
                .collect(),
        ))
    }
}

#[derive(PartialEq, Debug)]
pub enum PossibleTopic<'a> {
    /// Topic matches exactly
    Exact,

    /// Possible match
    CloseMatch { topic: &'a str, distance: f64 },

    /// No match found
    Nothing,
}

// TODO: Change to BTreeMap
pub struct SearchResults<'a>(pub Vec<(&'a str, &'a Note)>);

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

    #[test]
    fn test_match_topic() {
        // Assume
        let mut book = Notebook::default();
        book.0.entry("csharp".to_string()).or_insert(vec![]);
        book.0.entry("rust".to_string()).or_insert(vec![]);
        book.0.entry("javascript".to_string()).or_insert(vec![]);
        let searcher = NotebookSearch::new();

        // Act
        let exact = searcher.match_topic("javascript", &book);
        let close_1 = searcher.match_topic("javscriptt", &book);
        let close_2 = searcher.match_topic("rust1", &book);
        let nothing = searcher.match_topic("sharrp", &book);

        // Assert
        assert_eq!(PossibleTopic::Exact, exact);
        assert_eq!(
            PossibleTopic::CloseMatch {
                topic: "javascript",
                distance: 0.8
            },
            close_1
        );
        assert_eq!(
            PossibleTopic::CloseMatch {
                topic: "rust",
                distance: 0.8
            },
            close_2
        );
        assert_eq!(PossibleTopic::Nothing, nothing);
    }
}
