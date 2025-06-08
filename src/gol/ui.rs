use super::cell::{Cell, CellState};
use super::pattern::{SavedPatterns, SelectedPattern};
use super::state::Playing;
use crate::gol::patterns_io::{save_pattern, save_patterns};
use crate::screens::Screen;
use crate::theme::widget;
use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::{prelude::*, ui::Val::*};

#[derive(Component)]
struct GameMenuRoot;

#[derive(Component)]
struct ClearButton;

#[derive(Component)]
struct PatternButtons;

#[derive(Component)]
pub struct PatternButton {
    pub pattern_name: String,
}
impl PatternButton {
    pub fn from_name(name: &str) -> Self {
        Self {
            pattern_name: name.to_string(),
        }
    }
}

#[derive(Component)]
struct DeletePatternButton;

#[derive(Component)]
struct SavePatternButton;

#[derive(Component)]
pub struct SellectedPatternButton;

#[derive(Component)]
pub struct UnsellectedPatternButton;

const PLAY_COLOR: Color = Color::srgb(0.5, 0.0, 0.0); // dark red
const PAUSE_COLOR: Color = Color::srgb(0.0, 0.5, 0.0); // dark green
const PATTERN_COLOR: Color = Color::srgb(0.5, 0.5, 0.5); // dark grey
const PATTERN_SELECTED_COLOR: Color = Color::srgb(0.8, 0.8, 0.5); // yellowish grey

fn spawn_ui(mut commands: Commands, saved: Res<SavedPatterns>, selected: Res<SelectedPattern>) {
    commands
        .spawn((widget::ui_root_right("GameMenu"), GameMenuRoot))
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
            spawn_pattern_buttons_in_parent(root, &saved, &selected);
        });
}

fn cleanup_game_menu(mut commands: Commands, query: Query<Entity, With<GameMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn refresh_pattern_buttons(
    mut commands: Commands,
    saved: Res<SavedPatterns>,
    selected: Res<SelectedPattern>,
    pattern_list_query: Query<(Entity, Option<&Children>), With<PatternButtons>>,
) {
    if !saved.is_changed() {
        return;
    }
    if pattern_list_query.is_empty() {
        return;
    }

    for (parent, children_opt) in &pattern_list_query {
        // Despawn all children of the PatternButtons node
        if let Some(children) = children_opt {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }
        // Now spawn new pattern buttons as children
        commands.entity(parent).with_children(|patterns_root| {
            spawn_pattern_buttons(patterns_root, &saved, &selected);
        });
    }
}

fn spawn_pattern_buttons_in_parent(
    parent: &mut ChildSpawnerCommands,
    saved: &SavedPatterns,
    selected: &SelectedPattern,
) {
    parent
        .spawn(build_pattern_buttons_bundle())
        .with_child(build_pattern_button_bundle())
        .with_children(|parent| {
            spawn_pattern_buttons(parent, saved, selected);
        });
}

fn spawn_pattern_buttons_in_root(
    commands: &mut Commands,
    saved: &SavedPatterns,
    selected: &SelectedPattern,
) {
    commands
        .spawn(build_pattern_buttons_bundle())
        .with_child(build_pattern_button_bundle())
        .with_children(|parent| {
            spawn_pattern_buttons(parent, saved, selected);
        });
}

fn build_pattern_buttons_bundle() -> impl Bundle {
    (
        PatternButtons,
        Node {
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
    )
}

fn build_pattern_button_bundle() -> impl Bundle {
    (
        Name::new("PatternLabel"),
        BackgroundColor(Color::NONE),
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Px(10.0),
            ..default()
        },
    )
}

fn spawn_pattern_buttons(
    patterns_root: &mut ChildSpawnerCommands,
    saved: &SavedPatterns,
    selected: &SelectedPattern,
) {
    patterns_root.spawn((
        Text::new("Patterns:"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::BLACK),
        TextLayout::default(),
    ));

    // Collect and sort pattern names by cell count
    let mut patterns: Vec<_> = saved.0.iter().collect();
    patterns.sort_by_key(|(_, pattern)| pattern.cells.len());

    for (name, pattern) in patterns {
        let (preview_node, preview_color, preview_children) =
            pattern_preview(&to_state(&pattern.cells));
        let color = if name == &selected.0 {
            PATTERN_SELECTED_COLOR // Highlight the selected pattern
        } else {
            Color::NONE
        };
        let label = format!("{}.{}:", name, pattern.dir);
        let mut button = patterns_root.spawn((
            Button,
            PatternButton::from_name(name),
            Name::new(format!("PatternButton:{}", name)),
            BackgroundColor(color),
            Node {
                flex_direction: FlexDirection::Row,
                ..default()
            },
        ));
        button.with_children(|pattern_root| {
            pattern_root.spawn((
                Text::new(label),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::BLACK),
                TextLayout::default(),
            ));
            pattern_root.spawn((preview_node, preview_color, preview_children));
            if pattern.deletable {
                add_buttons_when_deletable(pattern_root, name);
            }
        });
        if name == &selected.0 {
            button.insert(SellectedPatternButton);
        }
    }
}

fn add_buttons_when_deletable(pattern_root: &mut ChildSpawnerCommands, name: &str) {
    pattern_root
        .spawn((
            Button,
            DeletePatternButton,
            Name::new(format!("DeletePatternButton:{}", name)),
            BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
            Node {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                margin: UiRect::left(Val::Px(8.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_child((
            Text::new("X"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::default(),
        ));
    pattern_root
        .spawn((
            Button,
            SavePatternButton,
            Name::new(format!("SavePatternButton:{}", name)),
            BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
            Node {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                margin: UiRect::left(Val::Px(4.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_child((
            Text::new("^"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::default(),
        ));
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
    mut commands: Commands,
    selected: Res<SelectedPattern>,
    mut query: Query<(Entity, &Name, &mut BackgroundColor), With<PatternButton>>,
) {
    if !selected.is_changed() {
        return;
    }
    for (entity, name, mut bg) in &mut query {
        if let Some(pattern_name) = name.as_str().strip_prefix("PatternButton:") {
            if pattern_name == selected.0 {
                *bg = BackgroundColor(PATTERN_SELECTED_COLOR);
                commands.entity(entity).insert(SellectedPatternButton);
            } else {
                *bg = BackgroundColor(PATTERN_COLOR);
                commands.entity(entity).remove::<SellectedPatternButton>();
            }
        }
    }
}

fn handle_delete_pattern_buttons(
    mut saved: ResMut<SavedPatterns>,
    interaction_query: Query<
        (&Interaction, &Name),
        (Changed<Interaction>, With<DeletePatternButton>),
    >,
) {
    for (interaction, name) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(pattern_name) = name.as_str().strip_prefix("DeletePatternButton:") {
                saved.0.remove(pattern_name);
                save_patterns(saved);
                break;
            }
        }
    }
}

fn handle_save_pattern_buttons(
    saved: ResMut<SavedPatterns>,
    interaction_query: Query<
        (&Interaction, &Name),
        (Changed<Interaction>, With<SavePatternButton>),
    >,
) {
    for (interaction, name) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(pattern_name) = name.as_str().strip_prefix("SavePatternButton:") {
                save_pattern(pattern_name.to_string(), saved);
                break;
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
    app.add_systems(OnEnter(Screen::Gameplay), spawn_ui)
        .add_systems(OnExit(Screen::Gameplay), cleanup_game_menu)
        .add_systems(Update, (handle_play_button, handle_clear_button))
        .add_systems(Update, (handle_pattern_buttons, refresh_pattern_buttons))
        .add_systems(Update, (update_pattern_button_highlights,))
        .add_systems(
            Update,
            (handle_delete_pattern_buttons, handle_save_pattern_buttons),
        );
}
