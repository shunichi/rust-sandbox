extern crate rusqlite;
extern crate tempfile;

use rusqlite::Connection;
use std::io::*;
use std::fs::File;

// const DEFAULT_DB_PATH: &str = "Library/Application Support/Google/Chrome/Default/History";
const DEFAULT_DB_PATH: &str = "Library/Application Support/Google/Chrome/Profile 1/History";

struct Entry {
    title: String,
    url: String,
    // unixtime: i64,
    // count: i32,
}

macro_rules! output_error_and_exit {
    ($x: expr) => {
        {
            writeln!(&mut std::io::stderr(), "chist: error: {}", $x).unwrap();
            std::process::exit(1);
        }
    }
}

fn copy_history_to_temp(db_path: &str) -> Result<tempfile::NamedTempFile> {
    let mut tmp = tempfile::NamedTempFile::new()?;
    let mut file = File::open(db_path)?;
    std::io::copy(&mut file, &mut tmp)?;
    tmp.sync_all()?;
    Ok(tmp)
}

fn db_path() -> String {
    let mut path_buf = std::env::home_dir().unwrap();
    path_buf.push(DEFAULT_DB_PATH);
    path_buf.into_os_string().into_string().unwrap()
}

fn main() {
    let tmpfile = match copy_history_to_temp(&db_path()) {
        Ok(t) => t,
        Err(error) => output_error_and_exit!(error),
    };
    let conn = match Connection::open(tmpfile.path()) {
        Ok(c) => c,
        Err(error) => output_error_and_exit!(error),
    };
    let mut stmt = conn.prepare("select MAX(title) as title, MAX(urls.url) as url, MAX((visits.visit_time - 11676312000000000)/1000/1000) as unixtime, COUNT(urls.id) as count from visits inner join urls on visits.url = urls.id group by urls.url order by count desc").unwrap();
    let iter = stmt.query_map(&[], |row| {
            Entry {
                title: row.get(0),
                url: row.get(1),
                // unixtime: row.get(2),
                // count: row.get(3),
            }
        })
        .unwrap();

    for e in iter {
        let entry = e.unwrap();
        println!("{}|{}", entry.title, entry.url);
    }
}
