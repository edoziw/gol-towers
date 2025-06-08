use crate::{
    AppSystems, PausableSystems,
    gol::{
        cell::{Cell, CellState},
        grid::{GRID_HEIGHT, GRID_WIDTH, grid_to_world},
        interaction::{find_pattern, place_pattern},
        pattern::{Dir, SavedPatterns},
    },
    screens::Screen,
};
use bevy::prelude::*;
use rand::prelude::*;

const REGION_DEFAULT_HEIGHT: usize = GRID_HEIGHT / 5;

pub fn populate_player_region(mut cells: Query<&mut Cell>) {
    let mut rng = rand::thread_rng();

    for _ in 0..5 {
        let x = rng.gen_range(0..(GRID_WIDTH - 1));
        let y = rng.gen_range((GRID_HEIGHT - REGION_DEFAULT_HEIGHT)..(GRID_HEIGHT - 1));
        for dx in 0..2 {
            for dy in 0..2 {
                if let Some(mut cell) = cells.iter_mut().find(|c| c.x == x + dx && c.y == y + dy) {
                    cell.state = CellState::Alive;
                }
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct AiGliderTimer(Timer);

pub fn ai_spawn_glider_timer(
    mut cells: Query<(&mut Sprite, &mut Cell, &Transform)>,
    mut timer: ResMut<AiGliderTimer>,
    time: Res<Time>,
    //mut commands: Commands,
    saved: Res<SavedPatterns>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        // Pick random x in AI region
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..(GRID_WIDTH - 3));
        let y = rng.gen_range(0..REGION_DEFAULT_HEIGHT);

        // Pick SE or SW
        let dir = if rng.gen_bool(0.5) { Dir::SE } else { Dir::SW };

        // Place a glider pattern at (x, y) with direction
        if let Some(pattern) = find_pattern(saved.as_ref(), &"glider".to_string()) {
            let mut rotated = pattern.clone();
            rotated.change_heading(dir);
            let world_pos = grid_to_world(x, y);
            place_pattern(&mut cells, &pattern, world_pos);
            // You will need a function to place a pattern at grid coords (x, y)
            // e.g., place_pattern_at_grid(&mut cells, &pattern.cells, x, y);
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(AiGliderTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )))
    .add_systems(OnEnter(Screen::Gameplay), populate_player_region)
    .add_systems(
        FixedUpdate,
        ai_spawn_glider_timer
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}
