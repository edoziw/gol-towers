use super::cell::{Cell, CellState};
use super::pattern::{SavedPatterns, SelectedPattern};
use super::state::Playing;
use crate::screens::Screen;
use crate::theme::widget;
use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::{prelude::*, ui::Val::*};

#[derive(Component)]
struct ClearButton;

#[derive(Component)]
struct PatternButton;

const PLAY_COLOR: Color = Color::srgb(0.5, 0.0, 0.0); // dark red
const PAUSE_COLOR: Color = Color::srgb(0.0, 0.5, 0.0); // dark green
const PATTERN_COLOR: Color = Color::srgb(0.5, 0.5, 0.5); // dark grey
const PATTERN_SELECTED_COLOR: Color = Color::srgb(0.8, 0.8, 0.5); // yellowish grey

fn setup_ui(mut commands: Commands, saved: Res<SavedPatterns>, selected: Res<SelectedPattern>) {
    commands
        .spawn((widget::ui_root_right("GameMenu"),))
        .with_children(|root| {
            root.spawn((
                Button,
                ClearButton,
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
            ))
            .with_children(|clear_button| {
                // Add a label to the button
                clear_button.spawn((
                    Text::new("Clear"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout::default(),
                ));
            });
            root.spawn((
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(200.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Px(20.0),
                    margin: UiRect {
                        left: Val::Px(16.0),
                        top: Val::Px(16.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("PatternDropdown"),
            ));
            root.spawn((
                Name::new("PatternLabel"),
                BackgroundColor(Color::NONE),
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Px(10.0),
                    ..default()
                },
            ))
            .with_children(|patterns_root| {
                // Add a button for each pattern
                patterns_root.spawn((
                    Text::new("Patterns:"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                    TextLayout::default(),
                ));
                for (name, pattern) in &saved.0 {
                    let (preview_node, preview_color, preview_children) =
                        pattern_preview(&to_state(pattern));
                    let color = if name == &selected.0 {
                        PATTERN_SELECTED_COLOR // Highlight the selected pattern
                    } else {
                        Color::NONE
                    };
                    patterns_root
                        .spawn((
                            Button,
                            PatternButton, // <-- Add this marker so systems work
                            Name::new(format!("PatternButton:{}", name)),
                            BackgroundColor(color),
                            Node {
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                        ))
                        .with_children(|pattern_root| {
                            pattern_root.spawn((
                                Text::new(format!("{}:", name)),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                                TextLayout::default(),
                            ));
                            pattern_root.spawn((preview_node, preview_color, preview_children));
                        });
                }
            });
        });
}

fn to_state(pattern: &Vec<(i32, i32)>) -> Vec<Vec<CellState>> {
    // Convert the pattern to a 2D grid of CellState
    let max_x = pattern.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let max_y = pattern.iter().map(|(_, y)| *y).max().unwrap_or(0);

    let mut grid = vec![vec![CellState::Dead; (max_x + 1) as usize]; (max_y + 1) as usize];

    for &(x, y) in pattern {
        if x >= 0 && y >= 0 {
            grid[y as usize][x as usize] = CellState::Alive;
        }
    }

    grid
}

fn pattern_preview(
    pattern: &Vec<Vec<CellState>>,
) -> (
    Node,
    BackgroundColor,
    SpawnRelatedBundle<ChildOf, Vec<(Node, BackgroundColor)>>,
) {
    // Adjust these for preview size
    let cell_size = 8.0;
    let rows = pattern.len();
    let cols = pattern.get(0).map_or(0, |row| row.len());

    let mut children = Vec::new();
    for (y, row) in pattern.iter().enumerate() {
        for (x, state) in row.iter().enumerate() {
            let color = match state {
                CellState::Alive => Color::BLACK,
                CellState::Dead => Color::NONE,
            };
            children.push((
                Node {
                    width: Val::Px(cell_size),
                    height: Val::Px(cell_size),
                    position_type: PositionType::Absolute,
                    left: Val::Px(x as f32 * cell_size),
                    top: Val::Px(y as f32 * cell_size),
                    ..default()
                },
                BackgroundColor(color),
            ));
        }
    }

    (
        Node {
            width: Val::Px(cols as f32 * cell_size),
            height: Val::Px(rows as f32 * cell_size),
            position_type: PositionType::Relative,
            ..default()
        },
        BackgroundColor(Color::NONE),
        Children::spawn(children),
    )
}

fn handle_pattern_buttons(
    mut interaction_query: Query<
        (&Interaction, &Name),
        (Changed<Interaction>, With<PatternButton>),
    >,
    mut selected: ResMut<SelectedPattern>,
) {
    for (interaction, name) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(pattern_name) = name.as_str().strip_prefix("PatternButton:") {
                selected.0 = pattern_name.to_string();
                // output to console the selected pattern
                info!("Selected pattern: {}", selected.0);
            }
        }
    }
}

fn update_pattern_button_highlights(
    selected: Res<SelectedPattern>,
    mut query: Query<(&Name, &mut BackgroundColor), With<PatternButton>>,
) {
    if !selected.is_changed() {
        return;
    }
    for (name, mut bg) in &mut query {
        if let Some(pattern_name) = name.as_str().strip_prefix("PatternButton:") {
            if pattern_name == selected.0 {
                *bg = BackgroundColor(PATTERN_SELECTED_COLOR);
            } else {
                *bg = BackgroundColor(PATTERN_COLOR);
            }
        }
    }
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
    query: Query<(&Interaction, &Name), (Changed<Interaction>, With<ClearButton>)>,
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

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup_ui)
        .add_systems(
            Update,
            (
                handle_play_button,
                handle_clear_button,
                handle_pattern_buttons,
                update_pattern_button_highlights,
            ),
        );
}
