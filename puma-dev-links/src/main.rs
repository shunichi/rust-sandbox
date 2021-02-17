use std::{cmp::Ordering, fs};
use dirs;

enum EntryType {
    Link { target: String },
    Port { port: i32 },
}

struct Entry {
    name: String,
    entry_type: EntryType,
}

fn entry_type_ord(entry: &Entry) -> i32 {
    match entry.entry_type {
        EntryType::Link { .. } => 0,
        EntryType::Port { .. } => 1,
    }
}

fn entry_cmp(e1: &Entry, e2: &Entry) -> Ordering {
    let o1 = entry_type_ord(e1);
    let o2 = entry_type_ord(e2);
    let ord = o1.cmp(&o2);
    if ord != Ordering::Equal {
        return ord;
    }
    match e1.entry_type {
        EntryType::Link { .. } => {
            match e2.entry_type {
                EntryType::Link { .. } => {
                    e1.name.cmp(&e2.name)
                },
                _ => panic!()
            }
        },
        EntryType::Port { port, .. } => {
            let port1 = port;
            match e2.entry_type {
                EntryType::Port { port, .. } => {
                    port1.cmp(&port)
                },
                _ => panic!()
            }
        }
    }
}

fn get_puma_dev_entries() -> std::io::Result<Vec<Entry>> {
    let mut dir_path = dirs::home_dir().unwrap();
    dir_path.push(".puma-dev");
    let mut vec = Vec::new();
    for dir_entry in  fs::read_dir(dir_path)? {
        let dir = dir_entry?;
        let file_type = dir.file_type()?;
        let file_name = dir.file_name().to_string_lossy().to_string();
        if file_type.is_symlink() {
            let target_path = fs::read_link(dir.path())?;
            vec.push(Entry { name: file_name, entry_type: EntryType::Link { target: target_path.to_string_lossy().to_string() } });
        } else if file_type.is_file() {
            let content = fs::read_to_string(dir.path())?;
            let first_line = content.lines().next().unwrap_or("");
            let port: i32 = first_line.parse::<i32>().unwrap();
            vec.push(Entry { name: file_name, entry_type: EntryType::Port { port: port } });
        }
    }
    vec.sort_by(|a, b| entry_cmp(a, b ));
    Ok(vec)
}

fn main() -> std::io::Result<()> {
    let entries = get_puma_dev_entries()?;
    if entries.len() == 0 {
        return Ok(());
    }

    let name_width = entries.iter().map(|e| e.name.len()).max().unwrap();
    for entry in entries {
        match entry.entry_type {
            EntryType::Link { target } => {
                println!("{:width$} -> {}", entry.name, target, width = name_width);
            },
            EntryType::Port { port } => {
                println!("{:width$} {}", entry.name, port, width = name_width);
            }
        }
    }
    Ok(())
}
