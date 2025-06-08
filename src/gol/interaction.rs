use super::{
    cell::{Cell, CellState},
    grid::CELL_SIZE,
    pattern::{SavedPatterns, SelectedPattern},
};
use crate::gol::{pattern::Pattern, patterns_io::load_patterns};
use crate::screens::Screen;
use bevy::prelude::*;

#[derive(Resource, Default)]
struct DragStart(Option<(Vec2, f64)>); // Store start position and time of drag

fn drag_start(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time<Fixed>>,
    mut drag_start: ResMut<DragStart>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Ok((camera, transform)) = camera_q.single() else {
        return;
    };
    let Ok(win) = windows.single() else {
        return;
    };
    let Some(pos) = win.cursor_position() else {
        return;
    };
    let Ok(world) = camera.viewport_to_world_2d(transform, pos) else {
        return;
    };
    drag_start.0 = Some((world, time.elapsed_secs_f64()));
}

fn drag_end_or_click(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time<Fixed>>,
    drag_start: Res<DragStart>,
    mut cells: Query<(&mut Sprite, &mut Cell, &Transform)>,
    selected: Res<SelectedPattern>,
    mut saved: ResMut<SavedPatterns>,
) {
    if !buttons.just_released(MouseButton::Left) {
        return;
    }
    let Some((start_pos, start_time)) = drag_start.0 else {
        return;
    };

    let Ok((camera, transform)) = camera_q.single() else {
        return;
    };
    let Ok(win) = windows.single() else {
        return;
    };
    let Some(pos) = win.cursor_position() else {
        return;
    };
    let Ok(end) = camera.viewport_to_world_2d(transform, pos) else {
        return;
    };

    let duration = time.elapsed_secs_f64() - start_time;
    let pattern_name = &selected.0;
    if duration < 1.0 && start_pos.distance(end) < CELL_SIZE {
        if selected.0 == "1x1" {
            // If the selected pattern is "1x1", toggle the cell at the clicked position
            toggle_cell(&mut cells, end);
        } else {
            // Otherwise, place pattern
            let Some(pattern) = find_pattern(saved.as_ref(), pattern_name) else {
                return;
            };
            place_pattern(&mut cells, pattern, end);
        }
        return;
    }

    let min = start_pos.min(end);
    let max = start_pos.max(end);

    let mut selected = vec![];
    for (_, cell, trans) in cells {
        let world_pos = trans.translation.truncate();
        if world_pos.x >= min.x
            && world_pos.x <= max.x
            && world_pos.y >= min.y
            && world_pos.y <= max.y
            && cell.state == CellState::Alive
        {
            let rel_x = (world_pos.x - min.x).round() as i32 / CELL_SIZE as i32;
            let rel_y = (world_pos.y - min.y).round() as i32 / CELL_SIZE as i32;
            selected.push((rel_x, rel_y));
        }
    }

    // For now, use a placeholder name; later prompt via UI
    let name = format!("Pattern{}", saved.0.len() + 1);
    saved.0.insert(
        name.clone(),
        Pattern::new_deleatable(name.clone(), selected.clone()),
    );

    println!("Saved pattern '{name}': {:?}", selected);
}

pub fn find_pattern<'a>(patterns: &'a SavedPatterns, pattern_name: &String) -> Option<&'a Pattern> {
    let result = patterns.0.get(pattern_name);
    if result.is_none() {
        println!("Pattern '{}' not found in saved patterns.", pattern_name);
    }
    result
}

pub fn place_pattern(
    cells: &mut Query<(&mut Sprite, &mut Cell, &Transform)>,
    pattern: &Pattern,
    world_pos: Vec2,
) {
    info!("Placing pattern '{}' at {world_pos:?}", pattern.name);
    for (x, y) in pattern.cells.iter() {
        let cell_pos = world_pos + Vec2::new(*x as f32 * CELL_SIZE, *y as f32 * CELL_SIZE);
        toggle_cell(cells, cell_pos);
    }
}

fn click_to_toggle_cell(
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    camera_q: Single<(&Camera, &GlobalTransform)>,
    mut cells: Query<(&mut Sprite, &mut Cell, &Transform)>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = *camera_q;
    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };
    toggle_cell(&mut cells, world_pos);
}

fn toggle_cell(cells: &mut Query<(&mut Sprite, &mut Cell, &Transform)>, world_pos: Vec2) {
    for (mut sprite, mut cell, transform) in cells.iter_mut() {
        let cell_pos = transform.translation.truncate();
        let half_size = CELL_SIZE / 2.0;
        let in_x = (world_pos.x - cell_pos.x).abs() < half_size;
        let in_y = (world_pos.y - cell_pos.y).abs() < half_size;

        if in_x && in_y {
            cell.state = if cell.state == CellState::Alive {
                CellState::Dead
            } else {
                CellState::Alive
            };

            sprite.color = match cell.state {
                CellState::Alive => Color::BLACK,
                CellState::Dead => Color::WHITE,
            };
            break;
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app //.add_systems(Update, click_to_toggle_cell)
        .add_systems(Update, (drag_start, drag_end_or_click))
        .add_systems(OnEnter(Screen::Splash), load_patterns)
        .insert_resource(DragStart::default())
        .insert_resource(SelectedPattern("1x1".to_string()))
        .insert_resource(SavedPatterns::default());
}
