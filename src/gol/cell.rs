use bevy::color::palettes::basic::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum CellState {
    Alive(CellType),
    Dead,
}
impl CellState {
    pub fn is_alive(&self) -> bool {
        matches!(self, CellState::Alive(_))
    }
    pub fn is_dead(&self) -> bool {
        matches!(self, CellState::Dead)
    }
    pub fn kind(&self) -> CellType {
        match self {
            CellState::Alive(kind) => *kind,
            CellState::Dead => CellType::Empty,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum RegionOwner {
    None,
    Player,
    AI,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum CellType {
    Tree,
    Water,
    Fire,
    Empty,
}
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl CellType {
    pub fn battle(&self, other: &Self) -> Outcome {
        match (self, other) {
            (CellType::Tree, CellType::Water)
            | (CellType::Water, CellType::Fire)
            | (CellType::Fire, CellType::Tree) => Outcome::Win,
            (CellType::Water, CellType::Tree)
            | (CellType::Fire, CellType::Water)
            | (CellType::Tree, CellType::Fire) => Outcome::Lose,
            _ => Outcome::Draw, // If they are the same or not in the cycle, return self
        }
    }
    pub fn colors(&self) -> Vec<Color> {
        match self {
            CellType::Tree => vec![GREEN.into(), MAROON.into()],
            CellType::Water => vec![BLUE.into(), AQUA.into()],
            CellType::Fire => vec![RED.into(), FUCHSIA.into()],
            CellType::Empty => vec![WHITE.into(), GRAY.into()],
        }
    }
    pub fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        *self.colors().choose(&mut rng).unwrap()
    }
}

#[derive(Component)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub state: CellState,
    pub region: RegionOwner,
    pub kind: CellType,
}
