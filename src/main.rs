extern crate greprs;

use greprs::arparser::*;
use greprs::file_parser::get_file_content;
use greprs::text_parser::search;

use std::env;
use std::error::Error;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments: Arguments = Arguments::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!(
        "Looking for {:?} in file {:?}\n",
        arguments.search_term, arguments.file_path
    );

    if let Err(e) = run(arguments) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(arguments: Arguments) -> Result<(), Box<Error>> {
    let file_content = match get_file_content(arguments.file_path) {
        Ok(file_content) => file_content,
        Err(err) => {
            panic!("Error opening the file {:?}", err);
        }
    };

    let _temp= search(arguments.search_term, &file_content);

    Ok(())
}
