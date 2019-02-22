
use uuid::Uuid;

use std::collections::HashMap;
use std::vec::Vec;

/// Represents the contents of a notebook file.
pub struct Notebook(HashMap<String, Vec<String>>);

pub struct Note {
    id: Uuid,
}