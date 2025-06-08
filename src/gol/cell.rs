use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum RegionOwner {
    None,
    Player,
    AI,
}

#[derive(Component)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub state: CellState,
    pub region: RegionOwner,
}
