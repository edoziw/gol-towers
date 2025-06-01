// GOL MVP in Bevy
use bevy::prelude::*;

use rand::prelude::*;
use rand::rng;

const GRID_WIDTH: usize = 64;
const GRID_HEIGHT: usize = 64;
const CELL_SIZE: f32 = 10.0;

#[derive(Component, Clone, Copy, PartialEq)]
enum CellState {
    Alive,
    Dead,
}

#[derive(Component)]
struct Cell {
    x: usize,
    y: usize,
    state: CellState,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let mut rng = rng();

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let state = if rng.random_bool(0.2) {
                CellState::Alive
            } else {
                CellState::Dead
            };

            let color = match state {
                CellState::Alive => Color::BLACK,
                CellState::Dead => Color::WHITE,
            };

            commands.spawn((
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
                children![Cell { x, y, state }],
            ));
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GOL MVP".to_string(),
                resolution: (800.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}
