use bevy::{prelude::*, time::Fixed};
use rand::{Rng, rng};
use std::collections::HashMap;

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

#[derive(Resource)]
struct Playing(bool);

fn setup_grid(mut commands: Commands) {
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
                if nx >= 0 && ny >= 0 && nx < GRID_WIDTH as i32 && ny < GRID_HEIGHT as i32 {
                    if grid[ny as usize][nx as usize] == CellState::Alive {
                        alive_neighbors += 1;
                    }
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

    for (mut sprite, mut cell, transform) in &mut cells {
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

#[derive(Component)]
struct PlayingButton;

const PLAY_COLOR: Color = Color::srgb(0.5, 0.0, 0.0); // dark red
const PAUSE_COLOR: Color = Color::srgb(0.0, 0.5, 0.0); // dark green

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            ..default()
        },
        Name::new("RootUI"),
        children![
            (
                Button,
                PlayingButton,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                Name::new("PlayPauseButton"),
                BackgroundColor(PLAY_COLOR), // dark red
                children![(
                    Text::new("Play"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout::default(),
                )]
            ),
            (
                Button,
                PlayingButton,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                Name::new("ClearButton"),
                BackgroundColor(PLAY_COLOR), // dark red
                children![(
                    Text::new("Clear"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout::default(),
                )]
            )
        ],
    ));
}

fn handle_play_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children, &Name),
        (Changed<Interaction>, With<Button>, With<Name>),
    >,
    mut text_query: Query<&mut Text>,
    mut playing: ResMut<Playing>,
) {
    for (interaction, mut bg_color, children, name) in &mut interaction_query {
        if *interaction == Interaction::Pressed && name.as_str() == "PlayPauseButton" {
            playing.0 = !playing.0;
            let label = if playing.0 { "Pause" } else { "Play" };
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.0 = label.to_string();
                }
            }
            *bg_color = if playing.0 {
                BackgroundColor(PAUSE_COLOR)
            } else {
                BackgroundColor(PLAY_COLOR)
            };
        }
    }
}

fn handle_clear_button(
    query: Query<(&Interaction, &Name), (Changed<Interaction>, With<Button>)>,
    mut cell_query: Query<(&mut Sprite, &mut Cell)>,
) {
    for (interaction, name) in query.iter() {
        if *interaction == Interaction::Pressed && name.as_str() == "ClearButton" {
            for (mut sprite, mut cell) in &mut cell_query {
                cell.state = CellState::Dead;
                sprite.color = Color::WHITE;
            }
        }
    }
}

fn is_playing(playing: Res<Playing>) -> bool {
    playing.0
}
#[derive(Resource, Default)]
struct DragStart(Option<(Vec2, f64)>); // Store start position and time of drag

#[derive(Resource)]
pub struct SavedPatterns(HashMap<String, Vec<(i32, i32)>>);

impl Default for SavedPatterns {
    fn default() -> Self {
        SavedPatterns(HashMap::from([
            ("1x1".to_string(), vec![(0, 0)]),
            ("2x2".to_string(), vec![(0, 0), (1, 0), (0, 1), (1, 1)]),
            (
                "glider".to_string(),
                vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
            ),
        ]))
    }
}

#[derive(Resource)]
struct SelectedPattern(String); // E.g., "glider", "2x2", or user-named

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

fn drag_end(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time<Fixed>>,
    drag_start: Res<DragStart>,
    cells: Query<(&Cell, &Transform)>,
    mut saved: ResMut<SavedPatterns>,
) {
    if !buttons.just_released(MouseButton::Left) {
        return;
    }
    let Some((start_pos, start_time)) = drag_start.0 else {
        return;
    };

    let duration = time.elapsed_secs_f64() - start_time;
    if duration < 1.0 {
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
    let Ok(end) = camera.viewport_to_world_2d(transform, pos) else {
        return;
    };
    let min = start_pos.min(end);
    let max = start_pos.max(end);

    let mut selected = vec![];
    for (cell, trans) in &cells {
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
    saved.0.insert(name.clone(), selected.clone());

    println!("Saved pattern '{name}': {:?}", selected);
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
        .add_systems(Startup, setup_grid)
        .insert_resource(Playing(false))
        .add_systems(Startup, setup_ui)
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .insert_resource(DragStart::default())
        .insert_resource(SavedPatterns::default())
        .insert_resource(SelectedPattern("1x1".to_string()))
        .add_systems(FixedUpdate, game_of_life_step.run_if(is_playing))
        .add_systems(Update, handle_play_button)
        .add_systems(Update, handle_clear_button)
        .add_systems(Update, click_to_toggle_cell)
        .add_systems(Update, (drag_start, drag_end))
        .run();
}
