use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Resource)]
pub struct SavedPatterns(pub HashMap<String, Pattern>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pattern {
    pub name: String,           // E.g., "glider", "2x2", or user-named
    pub cells: Vec<(i32, i32)>, // Coordinates of live cells relative to the top-left corner
    pub deletable: bool,        // Whether this pattern was defined by the user
    pub is_temp: bool,          // temp patterns are not saved
}

impl Pattern {
    pub fn new_deleatable<S: Into<String>>(name: S, cells: Vec<(i32, i32)>) -> Self {
        Pattern {
            name: name.into(),
            cells,
            deletable: true,
            is_temp: true, // new patterns are temporary until saved
        }
    }
}

impl Pattern {
    pub fn readonly_map_entry<S: Into<String>>(
        name: S,
        cells: Vec<(i32, i32)>,
    ) -> (String, Pattern) {
        let name_str = name.into();
        (
            name_str.clone(),
            Pattern {
                name: name_str,
                cells,
                deletable: false,
                is_temp: true, // Readonly patterns are not saved
            },
        )
    }
}

impl Default for SavedPatterns {
    fn default() -> Self {
        SavedPatterns(HashMap::from([
            Pattern::readonly_map_entry("1x1", vec![(0, 0)]),
            Pattern::readonly_map_entry("2x2", vec![(0, 0), (1, 0), (0, 1), (1, 1)]),
            Pattern::readonly_map_entry("glider", vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]),
        ]))
    }
}

#[derive(Resource)]
pub struct SelectedPattern(pub String); // E.g., "glider", "2x2", or user-named
