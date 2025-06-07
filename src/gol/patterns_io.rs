pub mod disk;
pub mod web_storage;

use crate::gol::pattern::{Pattern, SavedPatterns};
use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use crate::gol::patterns_io::disk::*;
#[cfg(target_arch = "wasm32")]
use crate::gol::patterns_io::web_storage::*;

pub fn load_patterns(mut saved: ResMut<SavedPatterns>) {
    #[cfg(target_arch = "wasm32")]
    let patterns = load_patterns_from_local_storage();

    #[cfg(not(target_arch = "wasm32"))]
    let patterns = load_patterns_from_file();

    for pattern in patterns {
        let existing = saved.0.get(&pattern.name);
        if existing.is_some() && !existing.unwrap().deletable {
            continue; // Skip loading patterns that not deletable
        }
        if existing.is_some() {
            saved.0.remove(&pattern.name); // Remove existing pattern
        }
        saved.0.insert(pattern.name.clone(), pattern);
    }
}

fn get_json_from_patterns_savable(saved: ResMut<SavedPatterns>) -> String {
    let patterns_to_save: Vec<Pattern> = saved.0.values().filter(|p| !p.is_temp).cloned().collect();
    serde_json::to_string(&patterns_to_save).unwrap_or_else(|_| {
        eprintln!("Failed to serialize patterns to JSON.");
        "[]".to_string() // Return an empty array if serialization fails
    })
}

pub fn save_pattern(name: String, mut saved: ResMut<SavedPatterns>) {
    mark_as_not_temp(name, &mut saved);
    save_patterns(saved);
}

fn mark_as_not_temp(name: String, saved: &mut SavedPatterns) {
    if let Some(pattern) = saved.0.get_mut(&name) {
        pattern.is_temp = false;
    } else {
        eprintln!("Pattern '{}' not found in saved patterns.", name);
    }
}

pub fn save_patterns(saved: ResMut<SavedPatterns>) {
    let json = get_json_from_patterns_savable(saved);

    #[cfg(target_arch = "wasm32")]
    save_patterns_to_local_storage(json);

    #[cfg(not(target_arch = "wasm32"))]
    save_patterns_to_file(json);
}
