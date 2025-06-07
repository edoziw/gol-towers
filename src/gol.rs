pub mod cell;
pub mod grid;
pub mod interaction;
pub mod pattern;
pub mod patterns_io;
pub mod state;
pub mod ui;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ui::plugin, interaction::plugin, grid::plugin))
        .insert_resource(state::Playing(false));
}
