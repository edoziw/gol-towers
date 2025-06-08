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
        // Pick random position in the AI region (top 1/5th of the grid)
        // and ensure the pattern fits on the grid.
        let mut rng = rand::thread_rng();

        if let Some(pattern) = find_pattern(saved.as_ref(), &"glider".to_string()) {
            // Pick SE or SW heading each spawn
            let dir = if rng.gen_bool(0.5) { Dir::SE } else { Dir::SW };
            let mut rotated = pattern.clone();
            rotated.change_heading(dir);

            // Calculate pattern dimensions for boundary checks
            let (pat_w, pat_h) = pattern.dimensions();
            let x_max = GRID_WIDTH.saturating_sub(pat_w);
            let y_min = GRID_HEIGHT - REGION_DEFAULT_HEIGHT;
            let y_max = GRID_HEIGHT.saturating_sub(pat_h);

            let x = rng.gen_range(0..=x_max);
            let y = rng.gen_range(y_min..=y_max);

            let world_pos = grid_to_world(x, y);
            place_pattern(&mut cells, &rotated, world_pos);
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
