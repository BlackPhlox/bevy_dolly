use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use bevy_enhanced_input::prelude::*;

pub struct DollyCursorGrab;
impl Plugin for DollyCursorGrab {
    fn build(&self, app: &mut App) {
        app.init_resource::<DollyCursorGrabConfig>()
            .add_plugins(EnhancedInputPlugin)
            .add_systems(
                Startup,
                (initial_grab_cursor, dolly_cursor_grab_input_setup),
            )
            .add_input_context::<CursorGrab>()
            //.add_systems(Update, cursor_grab.run_if(use_grab));
            ;
    }
}

#[derive(Resource)]
pub struct DollyCursorGrabConfig {
    pub enabled: bool,
    pub visible: bool,
}

impl Default for DollyCursorGrabConfig {
    fn default() -> Self {
        DollyCursorGrabConfig {
            enabled: true,
            visible: false,
        }
    }
}

fn use_grab(config: Res<DollyCursorGrabConfig>) -> bool {
    config.enabled
}

#[derive(Component)]
struct DollyCursorGrabAction;

fn dolly_cursor_grab_input_setup(commands: Commands) {
    //commands.spawn((DollyCursorGrabInputBundle::default(), DollyCursorGrabAction));
}

#[derive(Component)]
struct CursorGrab;

/*
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum GrabAction {
    Exit,
}


#[derive(Bundle)]
struct DollyCursorGrabInputBundle {
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
*/

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(cursor_options: &mut CursorOptions) -> bool {
    match cursor_options.grab_mode {
        CursorGrabMode::None => {
            cursor_options.grab_mode = CursorGrabMode::Confined;
            cursor_options.visible = false;
            false
        }
        _ => {
            cursor_options.grab_mode = CursorGrabMode::None;
            cursor_options.visible = true;
            true
        }
    }
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(
    mut cursor_options: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut config: ResMut<DollyCursorGrabConfig>,
) {
    config.visible = if !config.enabled {
        if let Ok(cursor_options) = &mut cursor_options.single_mut() {
            toggle_grab_cursor(cursor_options)
        } else {
            false
        }
    } else if let Ok(cursor_options) = &mut cursor_options.single_mut() {
        toggle_grab_cursor(cursor_options)
    } else {
        println!("Primary window not found for `initial_grab_cursor`!");
        false
    };
}

fn cursor_grab(
    mut cursor_options: Query<&mut CursorOptions, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
    //act_query: Query<&ActionState<GrabAction>, With<DollyCursorGrabAction>>,
    mut config: ResMut<DollyCursorGrabConfig>,
) {
    if let Ok(cursor_options) = &mut cursor_options.single_mut() {
        //if let Ok(grab_action) = act_query.get_single() {
        if keys.just_pressed(KeyCode::Escape) {
            config.visible = toggle_grab_cursor(cursor_options);
        }
        // This doesn't work:
        /*
        if grab_action.just_pressed(GrabAction::Exit) {
            config.visible = toggle_grab_cursor(window);
        }
        */
        //}
    }
}
