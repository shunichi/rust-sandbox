extern crate chist;

use chist::history;
use chist::cli;
use std::io::Write;

fn main() {
    let options = cli::parse_opts();
    if let Err(error) = history::history(&options.db_path, &options.separator) {
        writeln!(&mut std::io::stderr(), "chist: error: {}", &error).unwrap();
        std::process::exit(1);
    }
}
