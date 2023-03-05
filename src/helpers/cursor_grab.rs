use bevy::{ecs::schedule::ShouldRun, prelude::*, window::CursorGrabMode};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike, InputManagerBundle,
};

pub struct DollyCursorGrab;
impl Plugin for DollyCursorGrab {
    fn build(&self, app: &mut App) {
        app.init_resource::<DollyCursorGrabConfig>()
            .add_startup_system(initial_grab_cursor)
            .add_startup_system(dolly_cursor_grab_input_setup)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(use_grab)
                    .with_system(cursor_grab),
            );
    }
}

#[derive(Resource)]
pub struct DollyCursorGrabConfig {
    pub enabled: bool,
    pub visible: bool
}

impl Default for DollyCursorGrabConfig {
    fn default() -> Self {
        DollyCursorGrabConfig { enabled: true, visible: false }
    }
}

fn use_grab(config: Res<DollyCursorGrabConfig>) -> ShouldRun {
    if config.enabled {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

#[derive(Component)]
struct DollyCursorGrabAction;

fn dolly_cursor_grab_input_setup(mut commands: Commands) {
    commands.spawn((DollyCursorGrabInputBundle::default(), DollyCursorGrabAction));
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum GrabAction {
    Exit,
}

#[derive(Bundle)]
struct DollyCursorGrabInputBundle {
    #[bundle]
    input_manager: InputManagerBundle<GrabAction>,
}

impl Default for DollyCursorGrabInputBundle {
    fn default() -> Self {
        use GrabAction::*;
        let mut input_map = InputMap::default();

        input_map.insert(KeyCode::Escape, Exit);

        let input_manager = InputManagerBundle {
            input_map,
            action_state: ActionState::default(),
        };

        Self { input_manager }
    }
}

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) -> bool {
    match window.cursor_grab_mode() {
        CursorGrabMode::None => {
            window.set_cursor_grab_mode(CursorGrabMode::Confined);
            window.set_cursor_visibility(false);
            false
        }
        _ => {
            window.set_cursor_grab_mode(CursorGrabMode::None);
            window.set_cursor_visibility(true);
            true
        }
    }
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>, mut config: ResMut<DollyCursorGrabConfig>) {
    config.visible = if !config.enabled {
        if let Some(window) = windows.get_primary_mut() {
            toggle_grab_cursor(window)
        } else {
            false
        }
    } else if let Some(window) = windows.get_primary_mut() {
        toggle_grab_cursor(window)
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
        false
    };
}

fn cursor_grab(
    mut windows: ResMut<Windows>,
    keys: Res<Input<KeyCode>>,
    act_query: Query<&ActionState<GrabAction>, With<DollyCursorGrabAction>>,
    mut config: ResMut<DollyCursorGrabConfig>
) {
    if let Some(window) = windows.get_primary_mut() {
        if let Ok(grab_action) = act_query.get_single() {
            if keys.just_pressed(KeyCode::Escape) {
                config.visible = toggle_grab_cursor(window);
            }
            // This doesn't work:
            if grab_action.just_pressed(GrabAction::Exit) {
                config.visible = toggle_grab_cursor(window);
            }
        }
    }
}
