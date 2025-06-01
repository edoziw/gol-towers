use bevy::{prelude::*, time::Fixed};
use rand::{rng, Rng};

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
        children![(
            Button,
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
                    //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::default(),
            )]
        )],
    ));
}

fn toggle_play_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut playing: ResMut<Playing>,
) {
    for (interaction, mut bg_color, children) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
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

fn is_playing(playing: Res<Playing>) -> bool {
    playing.0
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
        .insert_resource(Playing(false))
        .add_systems(Startup, setup_ui)
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .add_systems(FixedUpdate, game_of_life_step.run_if(is_playing))
        .add_systems(Update, toggle_play_button)
        .run();
}
