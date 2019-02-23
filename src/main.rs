extern crate chrono;
extern crate clap;
extern crate colored;
extern crate dirs;
extern crate serde;
extern crate serde_yaml;

mod book;

use clap::{App, SubCommand, Arg};
use colored::*;
use book::{Note, NotebookFileStorage, NotebookStore};

fn main() {
    let matches = App::new("VNote")
        .version("0.1.0")
        .author("Willem Victor <wimpievictor@gmail.com>")
        .about("A command-line tool for taking micro notes")
        .subcommand(SubCommand::with_name("add")
            .about("adds a note to book")
            .arg(Arg::with_name("TOPIC")
                .required(true)
                .help("name of note topic"))
            .arg(Arg::with_name("NOTE")
                .required(true)
                .help("text content of note")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let topic = matches.value_of("TOPIC").unwrap();
        let note = matches.value_of("NOTE").unwrap();

        println!("  {} adding [{}] {}", "#".yellow(), topic, note);

        // First we ensure that we can create a note
        let note = Note::new(note.to_string());

        // Then we save it to disk
        let store = NotebookFileStorage::default();
        
        if let Err(err) = store.setup() {
            println!("Failed to initiate file storage: {:?}", err);
        }

        println!("  {} added {}", "✓".green(), note.id());
    }
}
