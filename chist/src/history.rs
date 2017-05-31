use std;
use rusqlite;
use tempfile;

struct Entry {
    title: String,
    url: String,
    // unixtime: i64,
    // count: i32,
}

fn copy_history_to_temp(db_path: &str) -> std::io::Result<tempfile::NamedTempFile> {
    let mut tmp = tempfile::NamedTempFile::new()?;
    let mut file = std::fs::File::open(db_path)?;
    std::io::copy(&mut file, &mut tmp)?;
    tmp.sync_all()?;
    Ok(tmp)
}

pub fn history(db_path: &str, separator: &str) -> Result<(), String> {
    let tmpfile = copy_history_to_temp(db_path).map_err(|e| e.to_string())?;
    let conn = rusqlite::Connection::open(tmpfile.path())
        .map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("select MAX(title) as title, MAX(urls.url) as url, MAX((visits.visit_time - 11676312000000000)/1000/1000) as unixtime, COUNT(urls.id) as count from visits inner join urls on visits.url = urls.id group by urls.url order by count desc").unwrap();
    let iter = stmt.query_map(&[], |row| {
            Entry {
                title: row.get(0),
                url: row.get(1),
                // unixtime: row.get(2),
                // count: row.get(3),
            }
        })
        .map_err(|e| e.to_string())?;

    for e in iter {
        let entry = e.unwrap();
        println!("{}{}{}", entry.title, separator, entry.url);
    }
    Ok(())
}
