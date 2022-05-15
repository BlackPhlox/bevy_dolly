use bevy::{ecs::schedule::ShouldRun, prelude::*};
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

pub struct DollyCursorGrabConfig {
    pub enabled: bool,
}

impl Default for DollyCursorGrabConfig {
    fn default() -> Self {
        DollyCursorGrabConfig { enabled: true }
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
    commands
        .spawn()
        .insert(DollyCursorGrabAction)
        .insert_bundle(DollyCursorGrabInputBundle::default());
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

        input_map.insert(Exit, KeyCode::Escape);

        let input_manager = InputManagerBundle {
            input_map,
            action_state: ActionState::default(),
        };

        Self { input_manager }
    }
}

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>, config: Res<DollyCursorGrabConfig>) {
    toggle_grab_cursor(windows.get_primary_mut().unwrap());
    if !config.enabled {
        toggle_grab_cursor(windows.get_primary_mut().unwrap());
    }
}

fn cursor_grab(
    mut windows: ResMut<Windows>,
    act_query: Query<&ActionState<GrabAction>, With<DollyCursorGrabAction>>,
) {
    let window = windows.get_primary_mut().unwrap();
    let grab_action = act_query.single();
    if grab_action.pressed(GrabAction::Exit) {
        toggle_grab_cursor(window);
    }
}
