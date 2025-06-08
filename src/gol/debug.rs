use bevy::prelude::*;
use bevy::time::Timer;
use bevy::window::PrimaryWindow;

// Add this resource to track the timer
#[derive(Resource, Default)]
pub struct MouseWorldPosTimer(pub Timer);

pub fn log_mouse_world_pos_system(
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut timer: ResMut<MouseWorldPosTimer>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    let window = if let Ok(w) = windows.single() {
        w
    } else {
        return;
    };
    let (camera, camera_transform) = if let Ok(pair) = camera_q.single() {
        pair
    } else {
        return;
    };

    if let Some(screen_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            info!("Mouse world position: {:?}", world_pos);
        } else {
            info!("Mouse not over world.");
        }
    } else {
        info!("No mouse position available.");
    }
}

// In your plugin function, add:
pub(super) fn plugin(app: &mut App) {
    app.insert_resource(MouseWorldPosTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )))
    .add_systems(FixedUpdate, log_mouse_world_pos_system);
}
