use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::gol::pattern::{Dir, SavedPatterns};
use crate::gol::ui::{PatternButton, SellectedPatternButton};
use crate::screens::Screen;

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
struct Aim;

#[derive(InputContext)]
struct Aiming;

fn bind_actions(
    trigger: Trigger<Binding<Aiming>>,
    settings: Res<AppSettings>,
    mut actions: Query<&mut Actions<Aiming>>,
) {
    let mut actions = actions.get_mut(trigger.target()).unwrap();
    actions
        .bind::<Aim>()
        .to((
            settings.keyboard.aim_keyboard,
            settings.keyboard.aim_gamepad,
        ))
        .with_modifiers((DeadZone::default(), SmoothNudge::default()));
}

#[derive(Resource)]
struct AppSettings {
    keyboard: KeyboardSettings,
}

#[derive(Debug)]
struct KeyboardSettings {
    aim_keyboard: Cardinal<KeyCode>,
    aim_gamepad: Axial<GamepadAxis>,
}
impl Default for KeyboardSettings {
    fn default() -> Self {
        Self {
            aim_keyboard: Cardinal::wasd_keys(),
            aim_gamepad: Axial::left_stick(),
        }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            keyboard: KeyboardSettings {
                ..KeyboardSettings::default()
            },
        }
    }
}

fn apply_aim(
    trigger: Trigger<Fired<Aim>>,
    query: Query<&PatternButton, With<SellectedPatternButton>>,
    mut saved: ResMut<SavedPatterns>,
) {
    info!("aim: triggered");

    let Ok(pattern_button) = query.single() else {
        info!("aim: button not found");
        return;
    };
    let vec2 = trigger.value;
    info!("apply_aim: vec2 = {:?}", vec2);
    let name = {
        let Some(pattern) = saved.0.get_mut(&pattern_button.pattern_name) else {
            error!("pattern not found: {}", pattern_button.pattern_name);
            return;
        };
        let dir: Dir = vec2.into();
        info!("apply_aim: dir = {:?}", dir);
        pattern.change_heading(dir);
        pattern.name.clone()
    };
    saved.set_changed();
    info!("Applying aim: {:?} to pattern: {}", vec2, name);
    //command.entity(entity).
}
fn start_aim(mut commands: Commands) {
    /*let Ok(entity) = query.single() else {
        info!("No GameMenuRoot entity found, cannot start aim system.");
        return;
    };*/
    commands.spawn((Player, Actions::<Aiming>::default()));
    info!("aim system activated");
}
pub(super) fn plugin(app: &mut App) {
    app //.add_systems(Update, click_to_toggle_cell)
        .insert_resource(AppSettings::default())
        .add_plugins(EnhancedInputPlugin)
        .add_input_context::<Aiming>()
        .add_systems(OnEnter(Screen::Gameplay), start_aim)
        .add_observer(bind_actions)
        .add_observer(apply_aim);
    //.insert_resource(InputMap::<Aiming>::default())
}
