use super::cell::{Cell, CellState};
use super::pattern::{SavedPatterns, SelectedPattern};
use super::state::Playing;
use bevy::prelude::*;

#[derive(Component)]
struct ClearButton;

const PLAY_COLOR: Color = Color::srgb(0.5, 0.0, 0.0); // dark red
const PAUSE_COLOR: Color = Color::srgb(0.0, 0.5, 0.0); // dark green
const PATTERN_COLOR: Color = Color::srgb(0.5, 0.5, 0.5); // dark grey

fn setup_ui(mut commands: Commands, saved: Res<SavedPatterns>, selected: Res<SelectedPattern>) {
    let mut pattern_children: Vec<(
        Button,
        Name,
        BackgroundColor,
        (Text, TextFont, TextColor, TextLayout),
    )> = Vec::new();

    // Add a label at the top
    pattern_children.push((
        Button,
        Name::new("PatternLabel"),
        BackgroundColor(Color::NONE),
        (
            Text::new(format!("Pattern: {}", selected.0)),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::default(),
        ),
    ));

    // Add a button for each pattern
    for name in saved.0.keys() {
        pattern_children.push((
            Button,
            Name::new(format!("PatternButton:{}", name)),
            BackgroundColor(PATTERN_COLOR),
            (
                Text::new(name.clone()),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::default(),
            ),
        ));
    }

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
                children![(
                    Text::new("Clear"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout::default(),
                )]
            ),
            (
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(40.0),
                    margin: UiRect {
                        left: Val::Px(16.0),
                        top: Val::Px(16.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("PatternDropdown"),
                Children::spawn(pattern_children),
            )
        ],
    ));
}

fn handle_pattern_buttons(
    mut interaction_query: Query<(&Interaction, &Name), (Changed<Interaction>, With<Button>)>,
    mut selected: ResMut<SelectedPattern>,
) {
    for (interaction, name) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(pattern_name) = name.as_str().strip_prefix("PatternButton:") {
                selected.0 = pattern_name.to_string();
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

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_ui).add_systems(
        Update,
        (
            handle_play_button,
            handle_clear_button,
            handle_pattern_buttons,
        ),
    );
}
