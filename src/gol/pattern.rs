use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct SavedPatterns(pub HashMap<String, Pattern>);

pub struct Pattern {
    pub name: String,           // E.g., "glider", "2x2", or user-named
    pub cells: Vec<(i32, i32)>, // Coordinates of live cells relative to the top-left corner
    pub deletable: bool,        // Whether this pattern was defined by the user
}

impl Pattern {
    pub fn new_deleatable<S: Into<String>>(name: S, cells: Vec<(i32, i32)>) -> Self {
        Pattern {
            name: name.into(),
            cells,
            deletable: true,
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
