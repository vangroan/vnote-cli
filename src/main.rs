extern crate chrono;
extern crate clap;
extern crate serde;
extern crate serde_yaml;

mod book;

use clap::{App, SubCommand, Arg};

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

    println!("{:?}", matches);

    if let Some(matches) = matches.subcommand_matches("add") {
        println!("NOTE: {}: {}", matches.value_of("TOPIC").unwrap(), matches.value_of("NOTE").unwrap());
    }
}
