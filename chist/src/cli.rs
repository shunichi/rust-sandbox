use std;
use clap::{App, Arg};

pub struct Options {
    pub separator: String,
    pub db_path: String,
}

#[cfg(target_os = "linux")]
const DEFAULT_DB_PATH: &str = ".config/google-chrome/Default/History";
#[cfg(target_os = "macos")]
const DEFAULT_DB_PATH: &str = "Library/Application Support/Google/Chrome/Default/History";

fn default_db_path() -> String {
    let mut path_buf = std::env::home_dir().unwrap();
    path_buf.push(DEFAULT_DB_PATH);
    path_buf.into_os_string().into_string().unwrap()
}

pub fn parse_opts() -> Options {
    let matches = App::new("chrome history dumper")
        .version(crate_version!())
        .arg(Arg::with_name("separator")
                 .short("s")
                 .long("separtor")
                 .value_name("SEP")
                 .help("custom separator")
                 .takes_value(true))
        .arg(Arg::with_name("db-path")
                 .long("db-path")
                 .value_name("DB_PATH")
                 .help("chrome history database path")
                 .takes_value(true))
        .get_matches();
    Options {
        separator: matches.value_of("separator").unwrap_or("|").to_string(),
        db_path: matches
            .value_of("db-path")
            .map(|s| s.to_string())
            .unwrap_or(default_db_path()),
    }
}
