use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct SavedPatterns(pub HashMap<String, Vec<(i32, i32)>>);

impl Default for SavedPatterns {
    fn default() -> Self {
        SavedPatterns(HashMap::from([
            ("2x2".to_string(), vec![(0, 0), (1, 0), (0, 1), (1, 1)]),
            (
                "glider".to_string(),
                vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
            ),
        ]))
    }
}

#[derive(Resource)]
pub struct SelectedPattern(pub String); // E.g., "glider", "2x2", or user-named
