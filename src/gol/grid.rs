use super::cell::{Cell, CellState};
use crate::{
    AppSystems, PausableSystems,
    gol::{
        cell::{CellType, RegionOwner},
        pattern::Dir,
    },
    screens::Screen,
};
use bevy::prelude::*;
use rand::prelude::*;

pub const GRID_WIDTH: usize = 64;
pub const GRID_HEIGHT: usize = 64;
pub const CELL_SIZE: f32 = 10.0;

const REGION_DEFAULT_HEIGHT: usize = GRID_HEIGHT / 5;
const fn region_default_height() -> usize {
    REGION_DEFAULT_HEIGHT
}
/// Convert grid coordinates to world coordinates using the same
/// transformation that [`setup_grid`] applies when spawning cells.
pub fn grid_to_world(x: usize, y: usize) -> Vec2 {
    Vec2::new(
        (x as f32 - GRID_WIDTH as f32 / 2.0) * CELL_SIZE,
        (y as f32 - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE,
    )
}

pub struct Region {
    pub bounds: Rect,
}
pub struct WorldRegion {
    pub bounds: Rect,
}
impl Region {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            bounds: Rect::new(x, y, width, height),
        }
    }
    pub fn from(dir: Dir, range_opt: Option<usize>) -> Self {
        let range = range_opt.unwrap_or(REGION_DEFAULT_HEIGHT);
        let mut bounds = Rect::new(0.0, 0.0, GRID_WIDTH as f32, GRID_HEIGHT as f32); // all
        match dir {
            Dir::S => {
                bounds.max.y = range as f32;
            }
            Dir::N => {
                bounds.min.y = GRID_HEIGHT as f32 - range as f32;
            }
            Dir::E => {
                //to test
                bounds.min.x = GRID_WIDTH as f32 - range as f32;
            }
            Dir::W => {
                //to test
                bounds.max.x = range as f32;
            }
            _ => {}
        };
        Self { bounds: bounds }
    }
    pub fn to_world(&self) -> WorldRegion {
        let world_min = Vec2::new(
            (self.bounds.min.x - GRID_WIDTH as f32 / 2.0) * CELL_SIZE,
            (self.bounds.min.y - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE,
        );
        let world_max = Vec2::new(
            (self.bounds.max.x - GRID_WIDTH as f32 / 2.0) * CELL_SIZE,
            (self.bounds.max.y - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE,
        );
        let result = WorldRegion {
            bounds: Rect {
                min: world_min,
                max: world_max,
            },
        };
        info!("Region to world: {:?} -> {:?}", self.bounds, result.bounds);
        result
    }
}

impl WorldRegion {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            bounds: Rect::new(x, y, width, height),
        }
    }
    pub fn to_random_pos(&self) -> Vec2 {
        let x = rand::thread_rng().gen_range(self.bounds.min.x..self.bounds.max.x);
        let y = rand::thread_rng().gen_range(self.bounds.min.y..self.bounds.max.y);
        Vec2::new(x, y)
    }
}

enum InitialCellState {
    Dead,
    Ramdom,
}

const INITIAL_CELL_STATE: InitialCellState = InitialCellState::Dead;

pub fn setup_grid(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let (state, kind) = match INITIAL_CELL_STATE {
                InitialCellState::Dead => (CellState::Dead, CellType::Empty),
                InitialCellState::Ramdom => {
                    if rng.gen_bool(0.2) {
                        (CellState::Alive, CellType::Fire)
                    } else {
                        (CellState::Dead, CellType::Empty)
                    }
                }
            };

            let region_default_height = GRID_HEIGHT / 5;
            // Assign region
            let region = if y >= GRID_HEIGHT - region_default_height {
                RegionOwner::Player
            } else if y < region_default_height {
                RegionOwner::AI
            } else {
                RegionOwner::None
            };

            commands.spawn((
                StateScoped(Screen::Gameplay),
                Sprite {
                    color: kind.color(),
                    custom_size: Some(Vec2::splat(CELL_SIZE)),
                    ..Default::default()
                },
                Transform::from_xyz(
                    (x as f32 - GRID_WIDTH as f32 / 2.0) * CELL_SIZE,
                    (y as f32 - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE,
                    0.0,
                ),
                Cell {
                    x,
                    y,
                    state,
                    region,
                    kind,
                },
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
    app.insert_resource(Time::<Fixed>::from_seconds(0.2))
        .add_systems(
            FixedUpdate,
            game_of_life_step
                .in_set(AppSystems::Update)
                .in_set(PausableSystems),
        );
}
