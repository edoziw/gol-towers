// GOL MVP in Bevy
use bevy::prelude::*;
use rand::prelude::*;
use rand::rng;

const GRID_WIDTH: usize = 64;
const GRID_HEIGHT: usize = 64;
const CELL_SIZE: f32 = 10.0;
const TIME_STEP: f32 = 0.2; // seconds

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

#[derive(Resource, Deref, DerefMut)]
struct TimeAccumulator(f32);

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

fn update_life(
    mut query: Query<(&mut Sprite, &mut Cell)>,
    mut accumulator: ResMut<TimeAccumulator>,
    time: Res<Time>,
) {
    accumulator.0 += time.delta().as_secs_f32();
    if accumulator.0 < TIME_STEP {
        return;
    }
    accumulator.0 = 0.0;

    let grid = query
        .iter()
        .map(|(_, cell)| ((cell.x, cell.y), cell.state))
        .collect::<std::collections::HashMap<_, _>>();

    for (mut sprite, mut cell) in &mut query {
        let mut alive_neighbors = 0;

        for dy in [-1isize, 0, 1] {
            for dx in [-1isize, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = cell.x as isize + dx;
                let ny = cell.y as isize + dy;
                if nx >= 0 && ny >= 0 {
                    if let Some(CellState::Alive) = grid.get(&(nx as usize, ny as usize)) {
                        alive_neighbors += 1;
                    }
                }
            }
        }

        let new_state = match (cell.state, alive_neighbors) {
            (CellState::Alive, 2 | 3) => CellState::Alive,
            (CellState::Dead, 3) => CellState::Alive,
            _ => CellState::Dead,
        };

        cell.state = new_state;
        sprite.color = match new_state {
            CellState::Alive => Color::BLACK,
            CellState::Dead => Color::WHITE,
        };
    }
}

fn main() {
    App::new()
        .insert_resource(TimeAccumulator(0.0))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GOL MVP".to_string(),
                resolution: (800.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, update_life)
        .run();
}
