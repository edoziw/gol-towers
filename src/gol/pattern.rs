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
    pub dir: Dir,               // Direction of the pattern
}

impl Pattern {
    pub fn new_deleatable<S: Into<String>>(name: S, cells: Vec<(i32, i32)>) -> Self {
        Pattern {
            name: name.into(),
            cells,
            deletable: true,
            is_temp: true, // new patterns are temporary until saved
            dir: Dir::Unknown,
        }
    }
    pub fn readonly_map_entry<S: Into<String>>(
        name: S,
        cells: Vec<(i32, i32)>,
        dir: Dir,
    ) -> (String, Pattern) {
        let name_str = name.into();
        (
            name_str.clone(),
            Pattern {
                name: name_str,
                cells,
                deletable: false,
                is_temp: true, // Readonly patterns are not saved
                dir: dir,
            },
        )
    }
}

impl Pattern {
    pub fn to_heading(&self, new_dir: Dir) -> Pattern {
        match self.dir {
            Dir::None | Dir::Unknown => self.clone(),
            Dir::N | Dir::E | Dir::S | Dir::W | Dir::NE | Dir::NW | Dir::SE | Dir::SW => {
                if new_dir == self.dir {
                    self.clone()
                } else {
                    Pattern {
                        name: self.name.clone(),
                        cells: rotate_from_a_to_b(self.cells.clone(), &self.dir, &new_dir),
                        deletable: self.deletable,
                        is_temp: self.is_temp,
                        dir: new_dir,
                    }
                }
            }
        }
    }
}

fn rotate_from_a_to_b(cells: Vec<(i32, i32)>, from: &Dir, to: &Dir) -> Vec<(i32, i32)> {
    match from {
        Dir::N => match to {
            Dir::N => rot_2d_0(&cells),
            Dir::E => rot_2d_90(&cells),
            Dir::S => rot_2d_180(&cells),
            Dir::W => rot_2d_270(&cells),
            _ => {
                eprintln!("Illegal rotation from {:?} to {:?}", from, to);
                cells
            }
        },
        Dir::E => match to {
            Dir::E => rot_2d_0(&cells),
            Dir::S => rot_2d_90(&cells),
            Dir::W => rot_2d_180(&cells),
            Dir::N => rot_2d_270(&cells),
            _ => {
                eprintln!("Illegal rotation from {:?} to {:?}", from, to);
                cells
            }
        },
        Dir::S => match to {
            Dir::S => rot_2d_0(&cells),
            Dir::W => rot_2d_90(&cells),
            Dir::N => rot_2d_180(&cells),
            Dir::E => rot_2d_270(&cells),
            _ => {
                eprintln!("Illegal rotation from {:?} to {:?}", from, to);
                cells
            }
        },
        Dir::W => match to {
            Dir::W => rot_2d_0(&cells),
            Dir::N => rot_2d_90(&cells),
            Dir::E => rot_2d_180(&cells),
            Dir::S => rot_2d_270(&cells),
            _ => {
                eprintln!("Illegal rotation from {:?} to {:?}", from, to);
                cells
            }
        },
        // Diagonal directions can have their own helpers if needed, or keep as before
        Dir::NE => match to {
            Dir::NE => rot_2d_0(&cells),
            Dir::SE => rot_2d_90(&cells),
            Dir::SW => rot_2d_180(&cells),
            Dir::NW => rot_2d_270(&cells),
            _ => {
                eprintln!("Illegal rotation from {:?} to {:?}", from, to);
                cells
            }
        },
        Dir::NW => match to {
            Dir::NW => rot_2d_0(&cells),
            Dir::NE => rot_2d_90(&cells),
            Dir::SW => rot_2d_270(&cells),
            Dir::SE => rot_2d_180(&cells),
            _ => {
                eprintln!("Illegal rotation from {:?} to {:?}", from, to);
                cells
            }
        },
        Dir::SE => match to {
            Dir::SE => rot_2d_0(&cells),
            Dir::NW => rot_2d_90(&cells),
            Dir::NE => rot_2d_180(&cells),
            Dir::SW => rot_2d_270(&cells),
            _ => {
                eprintln!("Illegal rotation from {:?} to {:?}", from, to);
                cells
            }
        },
        Dir::SW => match to {
            Dir::SW => rot_2d_0(&cells),
            Dir::SE => rot_2d_90(&cells),
            Dir::NW => rot_2d_180(&cells),
            Dir::NE => rot_2d_270(&cells),
            _ => {
                eprintln!("Illegal rotation from {:?} to {:?}", from, to);
                cells
            }
        },
        Dir::None | Dir::Unknown => {
            if from == to {
                cells
            } else {
                eprintln!("Illegal rotation from {:?} to {:?}", from, to);
                cells
            }
        }
    }
}

fn rot_2d_0(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    cells.to_vec()
}
fn rot_2d_90(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    cells.iter().map(|&(x, y)| (y, -x)).collect()
}
fn rot_2d_180(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    cells.iter().map(|&(x, y)| (-x, -y)).collect()
}
fn rot_2d_270(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    cells.iter().map(|&(x, y)| (-y, x)).collect()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Dir {
    None,
    Unknown,
    N,
    S,
    E,
    W,
    NE,
    SE,
    SW,
    NW,
}

impl Default for SavedPatterns {
    fn default() -> Self {
        SavedPatterns(HashMap::from([
            Pattern::readonly_map_entry("1x1", vec![(0, 0)], Dir::None),
            Pattern::readonly_map_entry("2x2", vec![(0, 0), (1, 0), (0, 1), (1, 1)], Dir::None),
            Pattern::readonly_map_entry(
                "glider",
                vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
                Dir::NE,
            ),
            Pattern::readonly_map_entry("blinker", vec![(0, 1), (1, 1), (2, 1)], Dir::None),
            Pattern::readonly_map_entry(
                "toad",
                vec![(0, 1), (1, 1), (2, 1), (1, 2), (2, 2)],
                Dir::None,
            ),
            Pattern::readonly_map_entry(
                "beacon",
                vec![(0, 0), (1, 0), (0, 1), (2, 2), (2, 3), (3, 3)],
                Dir::None,
            ),
            Pattern::readonly_map_entry(
                "LWSS",
                vec![
                    (1, 0),
                    (4, 0),
                    (0, 1),
                    (0, 2),
                    (4, 2),
                    (0, 3),
                    (1, 3),
                    (2, 3),
                    (3, 3),
                ],
                Dir::W,
            ),
        ]))
    }
}

#[derive(Resource)]
pub struct SelectedPattern(pub String); // E.g., "glider", "2x2", or user-named
