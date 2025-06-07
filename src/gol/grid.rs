use super::cell::{Cell, CellState};
use crate::{AppSystems, PausableSystems, screens::Screen};
use bevy::prelude::*;
use rand::prelude::*;

const GRID_WIDTH: usize = 64;
const GRID_HEIGHT: usize = 64;
pub const CELL_SIZE: f32 = 10.0;

pub fn setup_grid(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let state = if rng.gen_bool(0.2) {
                CellState::Alive
            } else {
                CellState::Dead
            };

            let color = match state {
                CellState::Alive => Color::BLACK,
                CellState::Dead => Color::WHITE,
            };

            commands.spawn((
                StateScoped(Screen::Gameplay),
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(CELL_SIZE)),
                    ..Default::default()
                },
                Transform::from_xyz(
                    (x as f32 - GRID_WIDTH as f32 / 2.0) * CELL_SIZE,
                    (y as f32 - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE,
                    0.0,
                ),
                Cell { x, y, state },
            ));
        }
    }
}

fn game_of_life_step(mut query: Query<(&mut Sprite, &mut Cell)>) {
    let mut grid = vec![vec![CellState::Dead; GRID_WIDTH]; GRID_HEIGHT];

    // Copy current state
    for cell in query.iter() {
        grid[cell.1.y][cell.1.x] = cell.1.state;
    }

    for (mut sprite, mut cell) in query.iter_mut() {
        let mut alive_neighbors = 0;
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = cell.x as i32 + dx;
                let ny = cell.y as i32 + dy;
                if nx >= 0
                    && ny >= 0
                    && nx < GRID_WIDTH as i32
                    && ny < GRID_HEIGHT as i32
                    && grid[ny as usize][nx as usize] == CellState::Alive
                {
                    alive_neighbors += 1;
                }
            }
        }

        let next_state = match (cell.state, alive_neighbors) {
            (CellState::Alive, 2..=3) => CellState::Alive,
            (CellState::Dead, 3) => CellState::Alive,
            _ => CellState::Dead,
        };

        if next_state != cell.state {
            cell.state = next_state;
            sprite.color = match next_state {
                CellState::Alive => Color::BLACK,
                CellState::Dead => Color::WHITE,
            };
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app //.add_systems(Startup, setup_grid)
        .insert_resource(Time::<Fixed>::from_seconds(0.2))
        .add_systems(
            FixedUpdate,
            game_of_life_step
                .in_set(AppSystems::Update)
                .in_set(PausableSystems),
        );
}
