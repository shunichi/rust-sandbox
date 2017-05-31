extern crate rusqlite;
extern crate tempfile;

mod history;

use std::io::Write;

fn main() {
    if let Err(error) = history::history() {
        writeln!(&mut std::io::stderr(), "chist: error: {}", &error).unwrap();
        std::process::exit(1);
    }
}
