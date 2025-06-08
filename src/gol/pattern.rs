use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

use crate::gol::grid::Region;

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
                dir,
            },
        )
    }
}

impl Pattern {
    pub fn change_heading(&mut self, new_dir: Dir) {
        match self.dir {
            Dir::N | Dir::E | Dir::S | Dir::W => match new_dir {
                Dir::NW | Dir::NE | Dir::SW | Dir::SE => {
                    info!(
                        "Pattern '{}' is currently facing {:?}, changing to or from diagonal {:?} is not allowed",
                        self.name, self.dir, new_dir
                    );
                    return;
                }
                _ => {}
            },
            Dir::NE | Dir::NW | Dir::SE | Dir::SW => match new_dir {
                Dir::N | Dir::E | Dir::S | Dir::W => {
                    info!(
                        "Pattern '{}' is currently facing {:?}, changing to or from diagonal {:?} is not allowed",
                        self.name, self.dir, new_dir
                    );
                    return;
                }
                _ => {}
            },
            _ => {}
        }
        match self.dir {
            Dir::None | Dir::Unknown => {}
            Dir::N | Dir::E | Dir::S | Dir::W | Dir::NE | Dir::NW | Dir::SE | Dir::SW => {
                if new_dir == self.dir {
                    info!("Pattern '{}' is already facing {:?}", self.name, self.dir);
                } else {
                    info!(
                        "Pattern '{}' changing direction from {:?} to {:?}",
                        self.name, self.dir, new_dir
                    );
                    self.cells = rotate_from_a_to_b(self.cells.clone(), &self.dir, &new_dir);
                    self.dir = new_dir;
                    info!(
                        "Pattern '{}' changed direction to {:?} with cells {:?}",
                        self.name, self.dir, self.cells
                    );
                }
            }
        }
    }

    /// Returns the width and height of the pattern in cells.
    ///
    /// The pattern's cells are guaranteed to have non-negative
    /// coordinates, so the dimensions are the maximum x and y values
    /// plus one.
    fn dimensions(&self) -> (usize, usize) {
        let max_x = self.cells.iter().map(|(x, _)| *x).max().unwrap_or(0);
        let max_y = self.cells.iter().map(|(_, y)| *y).max().unwrap_or(0);
        ((max_x + 1) as usize, (max_y + 1) as usize)
    }
    pub fn to_region_that_accepts_my_cells(&self, given: &Region) -> Region {
        let (w, h) = self.dimensions();
        Region::new(
            given.bounds.min.x,
            given.bounds.min.y,
            given.bounds.max.x - w as f32,
            given.bounds.max.y - h as f32,
        )
    }
}

fn rotate_from_a_to_b(cells: Vec<(i32, i32)>, from: &Dir, to: &Dir) -> Vec<(i32, i32)> {
    let rotated = match from {
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
            Dir::SE => rot_2d_180(&cells),
            Dir::SW => rot_2d_270(&cells),
            _ => {
                eprintln!("Illegal rotation from {:?} to {:?}", from, to);
                cells
            }
        },
        Dir::SE => match to {
            Dir::SE => rot_2d_0(&cells),
            Dir::SW => rot_2d_90(&cells),
            Dir::NW => rot_2d_180(&cells),
            Dir::NE => rot_2d_270(&cells),
            _ => {
                eprintln!("Illegal rotation from {:?} to {:?}", from, to);
                cells
            }
        },
        Dir::SW => match to {
            Dir::SW => rot_2d_0(&cells),
            Dir::NW => rot_2d_90(&cells),
            Dir::NE => rot_2d_180(&cells),
            Dir::SE => rot_2d_270(&cells),
            _ => {
                eprintln!("Illegal rotation from {:?} to {:?}", from, to);
                cells
            }
        },
        Dir::None | Dir::Unknown => {
            if from != to {
                eprintln!("Illegal rotation from {:?} to {:?}", from, to);
            }
            cells
        }
    };
    translate_to_positive_coordinates(&rotated)
}

fn translate_to_positive_coordinates(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let min_x = cells.iter().map(|(x, _)| *x).min().unwrap_or(0);
    let min_y = cells.iter().map(|(_, y)| *y).min().unwrap_or(0);
    cells.iter().map(|(x, y)| (x - min_x, y - min_y)).collect()
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

impl From<Vec2> for Dir {
    fn from(vec: Vec2) -> Self {
        if vec.x == 0.0 {
            if vec.y == 0.0 {
                return Dir::None;
            } else if vec.y < 0.0 {
                return Dir::S;
            } else {
                return Dir::N;
            }
        }
        if vec.y == 0.0 {
            if vec.x < 0.0 {
                return Dir::W;
            } else {
                return Dir::E;
            }
        }
        if vec.x > 0.0 {
            if vec.y < 0.0 { Dir::SE } else { Dir::NE }
        } else if vec.y < 0.0 {
            Dir::SW
        } else {
            Dir::NW
        }
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Dir::None => "*",
            Dir::Unknown => "?",
            Dir::N => "N",
            Dir::S => "S",
            Dir::E => "E",
            Dir::W => "W",
            Dir::NE => "NE",
            Dir::SE => "SE",
            Dir::SW => "SW",
            Dir::NW => "NW",
        };
        write!(f, "{}", s)
    }
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
                Dir::S,
            ),
            Pattern::readonly_map_entry(
                "beacon",
                vec![(0, 0), (1, 0), (0, 1), (2, 2), (2, 3), (3, 3)],
                Dir::SW,
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
            Pattern::readonly_map_entry(
                "R-Pentomino",
                vec![(2, 1), (1, 2), (2, 2), (2, 3), (3, 3)],
                Dir::W,
            ),
            Pattern::readonly_map_entry(
                "face", //https://www.reddit.com/r/gameoflife/comments/g6u4m6/interesting_face_pattern_in_conways_game_of_life/
                vec![
                    (3, 2),
                    (2, 3),
                    (4, 3),
                    (1, 4),
                    (3, 4),
                    (5, 4),
                    (1, 5),
                    (2, 5),
                    (4, 5),
                    (5, 5),
                    (1, 6),
                    (5, 6),
                ],
                Dir::None,
            ),
            Pattern::readonly_map_entry(
                "Gosper Glider Gun",
                vec![
                    (15, 7),
                    (16, 7),
                    (14, 8),
                    (18, 8),
                    (13, 9),
                    (19, 9),
                    (27, 9),
                    (3, 10),
                    (4, 10),
                    (13, 10),
                    (17, 10),
                    (19, 10),
                    (20, 10),
                    (25, 10),
                    (27, 10),
                    (3, 11),
                    (4, 11),
                    (13, 11),
                    (19, 11),
                    (23, 11),
                    (24, 11),
                    (14, 12),
                    (18, 12),
                    (23, 12),
                    (24, 12),
                    (37, 12),
                    (38, 12),
                    (15, 13),
                    (16, 13),
                    (23, 13),
                    (24, 13),
                    (37, 13),
                    (38, 13),
                    (25, 14),
                    (27, 14),
                    (27, 15),
                ],
                Dir::SE,
            ),
        ]))
    }
}

#[derive(Resource)]
pub struct SelectedPattern(pub String); // E.g., "glider", "2x2", or user-named
