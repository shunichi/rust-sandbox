use std::io;
use std::io::*;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut line_set = HashSet::new();
    for line in stdin.lock().lines() {
        match line {
            Ok(str) => {
                if !line_set.contains(str.as_str()) {
                    println!("{}", str);
                    line_set.insert(str);
                }
            }
            Err(error) => {
                writeln!(&mut std::io::stderr(), "more_uniq: error: {}", error).unwrap();
                ::std::process::exit(1);
            }
        }
    }
}
