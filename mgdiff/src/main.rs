extern crate yaml_rust;
extern crate postgres;
extern crate regex;
extern crate users;

use std::process::Command;
use std::env;
use yaml_rust::YamlLoader;
use std::fs::File;
use std::io::Read;
use postgres::{Connection, TlsMode};
use regex::Regex;
use std::collections::{HashSet, HashMap};
use users::{get_user_by_uid, get_current_uid};

fn current_user_name() -> String {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    user.name().to_string()
}

fn database_name() -> String {
    let mut file = File::open("config/database.yml").expect("database.yml not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];
    let database_name = doc["development"]["database"].as_str().unwrap();

    String::from(database_name)
}

fn to_human_string(s: &str) -> String {
    s.replace('_', " ")
}

const POSTGRES_PORT: usize = 5432;

fn schema_migrations() -> HashSet<String> {
    let db_name = database_name();
    let url = format!("postgres://{}@localhost:{}/{}", current_user_name(), POSTGRES_PORT, db_name);
    let conn = Connection::connect(url, TlsMode::None).unwrap();
    conn.query("SELECT version FROM schema_migrations", &[]).unwrap().into_iter()
        .map(|row| row.get::<usize, String>(0))
        .collect()
}

fn migration_diff(branch_name: &str) -> HashMap<String,String> {
    let output = Command::new("git")
        .args(&["diff", branch_name, "--name-status"])
        .output()
        .expect("git diff failed");

    if output.status.success() {
        let re = Regex::new(r"^A\s+db/migrate/(\d+)_(.+)\.rb").unwrap();
        let output = String::from_utf8_lossy(&output.stdout);
        let versions = output
            .lines()
            .filter_map(|l| 
                re.captures(l)
                    .map(|c| (c.get(1).unwrap().as_str().to_string(), to_human_string(c.get(2).unwrap().as_str()) ))
            );
        versions.collect()
    } else {
        HashMap::new()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("missing arguments");
        std::process::exit(1);
    }

    let s = schema_migrations();
    let d = migration_diff(&args[1]);
    let dkeys: HashSet<String> = d.keys().cloned().collect();
    let mut u: Vec<&str> = s.intersection(&dkeys).map(|v| v.as_str()).collect();
    u.sort();
    if !u.is_empty() {
        println!("{} migration(s) should be rollbacked!", u.len());
        println!("-------------------------------------------");
        for version in u {
            println!("{} {}", version, d[version]);
        }
    }
}
