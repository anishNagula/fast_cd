use std::{env, fs, path::Path};
use serde::{Deserialize, Serialize};
use serde_json;
use chrono::{NaiveDateTime, Utc};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataLine {
    path: String,
    ranking: i32,
    timestamp: NaiveDateTime,
}

fn load_db(db_path: &str) -> Vec<DataLine> {
    if !Path::new(db_path).exists() {
        fs::create_dir_all("/Users/anishnagula/.fast_cd").expect("Failed to create directory");
        fs::write(db_path, "[]").expect("Failed to write empty DB");
    }

    let json_string = fs::read_to_string(db_path).expect("Failed to read DB");
    serde_json::from_str(&json_string).unwrap_or_default()
}

fn save_db(db_path: &str, db: &Vec<DataLine>) {
    let json_string = serde_json::to_string_pretty(db).expect("Failed to serialize DB");
    fs::write(db_path, json_string).expect("Failed to write DB");
}

fn save_entry(db_path: &str, path: &str, db: &mut Vec<DataLine>) {
    if let Some(entry) = db.iter_mut().find(|e| e.path == path) {
        entry.ranking += 1;
        entry.timestamp = Utc::now().naive_utc();
    } else {
        db.push(DataLine {
            path: path.to_string(),
            ranking: 1,
            timestamp: Utc::now().naive_utc(),
        });
    }
    save_db(db_path, db);
}

fn main() {
    let input = env::args().nth(1).expect("Please provide a folder name or path");
    let db_path = "/Users/anishnagula/.fast_cd/data.json";

    let mut db = load_db(db_path);

    // Use fuzzy matching
    let matcher = SkimMatcherV2::default();
    let mut best_match: Option<&DataLine> = None;
    let mut best_score = 0;

    for entry in &db {
        if let Some(score) = matcher.fuzzy_match(&entry.path, &input) {
            if score > best_score {
                best_score = score;
                best_match = Some(entry);
            }
        }
    }

    let target_path = if let Some(entry) = best_match {
        entry.path.clone()
    } else {
        input.clone() // fallback to raw input
    };

    // Save/update the entry
    save_entry(db_path, &target_path, &mut db);

    println!("{}", target_path);
}
