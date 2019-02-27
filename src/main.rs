extern crate chrono;
#[macro_use]
extern crate clap;
extern crate colored;
extern crate dirs;
extern crate regex;
extern crate serde;
extern crate serde_yaml;

mod book;

use clap::{App, SubCommand, Arg};
use colored::*;
use book::{Note, NotebookFileStorage, NotebookStore};

use std::collections::HashMap;

fn main() {
    // Older Windows CMD does not support coloured output
    #[cfg(windows)] {
        if !ansi_term::enable_ansi_support().is_ok() {
            colored::control::set_override(false);
        }
    }

    let matches = App::new("VNote")
        .version(crate_version!())
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
        .subcommand(SubCommand::with_name("find")
            .about("searches for a note using a regular expression")
            .arg(Arg::with_name("PATTERN")
                .required(true)
                .help("regular expression for search"))
            .arg(Arg::with_name("topic")
                .short("t")
                .long("topic")
                .takes_value(true)
                .help("narrows search to a specific topic")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let topic = matches.value_of("TOPIC").unwrap();
        let note = matches.value_of("NOTE").unwrap();

        println!("  {} adding [{}] {}", "#".yellow(), topic, note);

        // First we ensure that we can create a note
        let note = Note::new(note.to_string());
        let id = note.id().clone();

        // Then we save it to disk
        let store = NotebookFileStorage::default();
        
        if let Err(err) = store.setup() {
            eprintln!(" {} failed to initiate file storage: {:?}", "!".red(), err);
        }

        // TODO: get notebook name from command line argument
        store.add_note(topic, note, None).expect(&format!(" {} failed to save notebook", "!".red()));

        println!("  {} added {}", "✓".green(), id);
    }

    if let Some(matches) = matches.subcommand_matches("find") {
        let pattern = matches.value_of("PATTERN").unwrap();
        let maybe_topic = matches.value_of("topic");

        println!("  {} searching...", "#".yellow());
        let store = NotebookFileStorage::default();
        // TODO: get notebook name from command line argument
        match store.scan_notes(pattern, None, maybe_topic) {
            Ok(results) => {

                if results.is_empty() {
                    println!("  {} no results found", "✓".green());
                } else {
                    println!("  {} results found", "✓".green());

                    // For display, group according to topics
                    let mut topic_map : HashMap<String, Vec<Note>> = HashMap::new();
                    for (topic, note) in results {
                        topic_map.entry(topic)
                            .or_insert(vec![])
                            .push(note);
                    }

                    // Iterate again to display
                    for (topic, notes) in topic_map {
                        println!("  {}", topic.green());
                        for note in notes {
                            // TODO: colour matched part of string
                            println!("   - {}", note.content());
                        }
                    }
                }
            }
            Err(err) => eprintln!(" {} failed to search notebook: {:?}", "!".red(), err)
        }
    }
}
