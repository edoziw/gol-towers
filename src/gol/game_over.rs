use bevy::color::palettes::basic::*;
use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    gol::score::{AiScore, PlayerScore},
    menus::Menu,
    screens::Screen,
    theme::widget::{self, ui_root_left},
};

#[derive(Resource, Default)]
pub struct BestScore(pub i32);

#[derive(Resource, Default)]
pub struct GameOverState {
    pub is_over: bool,
    pub message: String,
}

#[derive(Component)]
struct GameOverUi;

fn check_game_over(
    player_score: Res<PlayerScore>,
    ai_score: Res<AiScore>,
    mut best_score: ResMut<BestScore>,
    mut game_over: ResMut<GameOverState>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    // Update best score if needed
    if player_score.0 > best_score.0 {
        best_score.0 = player_score.0;
    }

    // Check win/lose conditions
    if ai_score.0 < 1 && !game_over.is_over && player_score.0 >= 1 {
        game_over.is_over = true;
        game_over.message = "You win!".to_string();
        next_screen.set(Screen::GameOver);
    } else if player_score.0 < -100 && !game_over.is_over {
        game_over.is_over = true;
        game_over.message = "You lose.".to_string();
        next_screen.set(Screen::GameOver);
    }
}

// UI system for Game Over screen
fn show_game_over_ui(
    game_over: Res<GameOverState>,
    player_score: Res<PlayerScore>,
    ai_score: Res<AiScore>,
    best_score: Res<BestScore>,
    mut commands: Commands,
) {
    if !game_over.is_over {
        return;
    }
    commands
        .spawn(ui_root_left("Game Over"))
        .insert(GameOverUi)
        .with_children(|root| {
            root.spawn(build_text(game_over.message.as_str(), YELLOW.into(), 40.0));
            root.spawn(build_text(
                &format!("Player Score: {}", player_score.0),
                Color::WHITE,
                24.0,
            ));
            root.spawn(build_text(
                &format!("AI Score: {}", ai_score.0),
                Color::WHITE,
                24.0,
            ));
            root.spawn(build_text(
                &format!("Best Score: {}", best_score.0),
                Color::WHITE,
                24.0,
            ));
            root.spawn(widget::button("Back", go_back_on_click));
        });
}

fn go_back_on_click(
    _: Trigger<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(Screen::Title);
    next_menu.set(Menu::Main);
}

fn despawn_game_over_ui(mut commands: Commands, game_over_ui: Query<Entity, With<GameOverUi>>) {
    for entity in game_over_ui.iter() {
        commands.entity(entity).despawn();
    }
}

fn build_text(text: &str, color: Color, size: f32) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(color),
    )
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(BestScore::default())
        .insert_resource(GameOverState::default())
        .add_systems(
            FixedUpdate,
            (check_game_over)
                .in_set(AppSystems::Update)
                .in_set(PausableSystems),
        )
        .add_systems(OnEnter(Screen::GameOver), show_game_over_ui)
        .add_systems(OnExit(Screen::GameOver), despawn_game_over_ui);
}
