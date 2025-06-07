use crate::gol::pattern::Pattern;
use bevy::prelude::*;
use serde_json::from_str;
use web_sys::window;

const PATTERNS_KEY: &'static str = "gol-patterns";

pub fn load_patterns_from_local_storage() -> std::vec::Vec<Pattern> {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        let value = storage.get_item(PATTERNS_KEY).unwrap_or(None);
        if let Some(json) = value {
            if let Ok(patterns) = from_str::<Vec<Pattern>>(&json) {
                return patterns;
            } else {
                eprintln!("Failed to parse patterns from local storage.");
            }
        } else {
            eprintln!("No patterns found in local storage.");
        }
    } else {
        eprintln!("Local storage is not available.");
    }
    vec![] // Return an empty vector if loading fails
}

pub fn save_patterns_to_local_storage(json: String) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.set_item(PATTERNS_KEY, &json);
    }
}
