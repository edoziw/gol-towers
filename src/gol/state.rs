use bevy::prelude::*;

#[derive(Resource)]
pub struct Playing(pub bool);

/*
pub fn is_playing(playing: Res<Playing>) -> bool {
    playing.0
}
    */
