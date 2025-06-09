use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    gol::{
        cell::{Cell, CellType},
        ui::GameMenuRoot,
    },
    screens::Screen,
    theme::widget::ui_root_left,
};

#[derive(Resource, Default)]
pub struct PlayerScore(pub i32);

#[derive(Resource, Default)]
pub struct AiScore(pub i32);

#[derive(Component)]
struct AiScoreDisplay;

#[derive(Component)]
struct PlayerScoreDisplay;

fn update_scores(
    cells: Query<&Cell>,
    mut player_score: ResMut<PlayerScore>,
    mut ai_score: ResMut<AiScore>,
) {
    let mut tree: i32 = 0;
    let mut water: i32 = 0;
    let mut fire: i32 = 0;
    for cell in cells.iter() {
        match cell.state.kind() {
            CellType::Tree => tree += 1,
            CellType::Water => water += 1,
            CellType::Fire => fire += 1,
            _ => {}
        }
    }
    player_score.0 = tree - water;
    ai_score.0 = fire;
}
const INSTRUCTIONS: &str = r#"Protect the plants
Don't waste water.

To add watter:
Click a pattern to and press the keys w,a,s,d to choose a direction.
Click to place the pattern.

Press p to pause or quit to title."#;

fn spawn_ui_scores(mut commands: Commands) {
    commands
        .spawn(ui_root_left("Scores"))
        .insert(GameMenuRoot)
        .with_children(|root| {
            root.spawn(build_instructions(INSTRUCTIONS));
            root.spawn(build_ui_score("Player"))
                .insert(PlayerScoreDisplay);
            root.spawn(build_ui_score("AI")).insert(AiScoreDisplay);
        });
}
fn build_instructions(instructions: &str) -> (Text, TextFont, TextColor, TextLayout) {
    (
        Text::new(instructions.to_string()),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::default(),
    )
}
fn build_ui_score(name: &str) -> (Text, TextFont, TextColor, TextLayout) {
    (
        Text::new(format!("{name} Score: ")),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::default(),
    )
}

fn update_score_ui(
    player_score: Res<PlayerScore>,
    ai_score: Res<AiScore>,
    mut player_text: Query<&mut Text, With<PlayerScoreDisplay>>,
    mut ai_text: Query<&mut Text, (With<AiScoreDisplay>, Without<PlayerScoreDisplay>)>,
) {
    if let Ok(mut text) = player_text.single_mut() {
        text.0 = format!("Player Score: {}", player_score.0);
    }
    if let Ok(mut text) = ai_text.single_mut() {
        text.0 = format!("AI Score: {}", ai_score.0);
    }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(PlayerScore::default())
        .insert_resource(AiScore::default())
        .add_systems(OnEnter(Screen::Gameplay), spawn_ui_scores)
        .add_systems(
            FixedUpdate,
            (update_scores, update_score_ui)
                .chain()
                .in_set(AppSystems::Update)
                .in_set(PausableSystems),
        );
}
