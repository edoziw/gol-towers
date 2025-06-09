pub mod cell;
pub mod debug;
pub mod game_over;
pub mod grid;
pub mod input;
pub mod interaction;
pub mod pattern;
pub mod patterns_io;
pub mod player;
pub mod score;
pub mod state;
pub mod ui;

use bevy::prelude::*;

use crate::{
    gol::{game_over::check_game_over, grid::setup_grid, player::populate_player_region},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        ui::plugin,
        interaction::plugin,
        grid::plugin,
        input::plugin,
        player::plugin,
        debug::plugin,
        score::plugin,
        game_over::plugin,
    ))
    .insert_resource(state::Playing(false))
    .add_systems(
        OnEnter(Screen::Gameplay),
        (setup_grid, populate_player_region, check_game_over).chain(),
    );
}
