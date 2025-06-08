use crate::{
    AppSystems, PausableSystems,
    gol::{
        cell::Cell,
        grid::Region,
        interaction::{find_pattern, place_pattern},
        pattern::{Dir, Pattern, SavedPatterns},
    },
};
use bevy::prelude::*;
use rand::prelude::*;

pub fn populate_player_region(
    mut cells: Query<(&mut Sprite, &mut Cell, &Transform)>,
    saved: Res<SavedPatterns>,
) {
    let Some(pattern_unrotated) = find_pattern(saved.as_ref(), &"2x2".to_string()) else {
        return;
    };

    let player_region: Region = Region::from(Dir::S, None);
    let dirs = vec![Dir::None];

    for _ in 0..5 {
        spawn_pattern_at_random_in_region(&mut cells, &pattern_unrotated, &player_region, &dirs);
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
    if !timer.0.just_finished() {
        return;
    }
    let Some(pattern_unrotated) = find_pattern(saved.as_ref(), &"glider".to_string()) else {
        return;
    };

    let ai_region: Region = Region::from(Dir::N, None);
    let dirs = vec![Dir::SE, Dir::SW];

    spawn_pattern_at_random_in_region(&mut cells, &pattern_unrotated, &ai_region, &dirs);
}

fn spawn_pattern_at_random_in_region(
    cells: &mut Query<(&mut Sprite, &mut Cell, &Transform)>,
    pattern_unrotated: &Pattern,
    region: &Region,
    dirs: &Vec<Dir>,
) {
    let dir = match dirs.choose(&mut rand::thread_rng()) {
        Some(dir) => dir.clone(),
        None => Dir::E, // No directions available
    };
    let mut pattern = pattern_unrotated.clone();
    pattern.change_heading(dir);

    let world_pos = pattern
        .to_region_that_accepts_my_cells(region)
        .to_world()
        .to_random_pos();

    place_pattern(cells, &pattern, world_pos);
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(AiGliderTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )))
    .add_systems(
        FixedUpdate,
        ai_spawn_glider_timer
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}
