use crate::gol::pattern::Pattern;
use bevy::prelude::*;
use serde_json::from_reader;
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

fn patterns_file_path() -> PathBuf {
    let tmp_dir = env::var("TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
    let mut path = PathBuf::from(tmp_dir);
    path.push("patterns.json");
    path
}

pub fn load_patterns_from_file() -> std::vec::Vec<Pattern> {
    if !patterns_file_path().exists() {
        eprintln!("Patterns file does not exist, returning empty patterns.");
        return vec![];
    }
    let file = File::open(patterns_file_path());
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        if let Ok(patterns) = from_reader::<_, Vec<Pattern>>(reader) {
            return patterns;
        } else {
            eprintln!("Failed to parse patterns from file.");
        }
    } else {
        eprintln!("Failed to open patterns file.");
    }
    vec![] // Return an empty vector if loading fails
}

pub fn save_patterns_to_file(json: String) {
    if let Ok(mut file) = File::create(patterns_file_path()) {
        let _ = write!(file, "{}", json);
    }
}
