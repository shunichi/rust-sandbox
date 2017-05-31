extern crate chist;

use chist::history;
use std::io::Write;

fn main() {
    if let Err(error) = history::history(&history::default_db_path()) {
        writeln!(&mut std::io::stderr(), "chist: error: {}", &error).unwrap();
        std::process::exit(1);
    }
}
