use std::collections::HashMap;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn load_db(db_path: &str) -> HashMap<String, String> {
    if !Path::new(db_path).exists() {
        fs::create_dir_all("/Users/anishnagula/.fast_cd").expect("Failed to create directory");
        fs::write(db_path, "").expect("Failed to write empty DB");
    }

    let file = fs::File::open(db_path).expect("Failed to open DB file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|line| {
            let mut parts = line.splitn(2, ':');
            Some((parts.next()?.to_string(), parts.next()?.to_string()))
        })
        .collect()
}

fn save_entry(db_path: &str, shortcut: &str, actual_path: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(db_path)
        .expect("Failed to open DB for writing");
    writeln!(file, "{}:{}", shortcut, actual_path).expect("Failed to write entry");
}

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a path");
    let db_path = "/Users/anishnagula/.fast_cd/data.db";

    let db = load_db(db_path);

    let shortcut = Path::new(&file_path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    if let Some(actual_path) = db.get(&shortcut) {
        // Existing shortcut: cd to stored path
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("cd '{}' && exec $SHELL", actual_path))
            .status()
            .expect("Failed to run cd");
    } else {
        // New shortcut: save and cd to file_path
        save_entry(db_path, &shortcut, &file_path);
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("cd '{}' && exec $SHELL", file_path))
            .status()
            .expect("Failed to run cd");
    }
}