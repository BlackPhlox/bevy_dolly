use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::JustPressedMany;

pub struct DollyCursorGrab;
impl Plugin for DollyCursorGrab {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<DollyCursorGrabConfig>()
            .add_startup_system(initial_grab_cursor.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(use_grab.system())
                    .with_system(cursor_grab.system()),
            );
    }
}

pub struct DollyCursorGrabConfig {
    pub enabled: bool,
    pub grab_keys: &'static [KeyCode],
}

impl Default for DollyCursorGrabConfig {
    fn default() -> Self {
        DollyCursorGrabConfig {
            enabled: true,
            grab_keys: &[KeyCode::Escape],
        }
    }
}

fn use_grab(config: Res<DollyCursorGrabConfig>) -> ShouldRun {
    if config.enabled {
        ShouldRun::Yes
    } else {
        ShouldRun::No
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
    keys: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    config: Res<DollyCursorGrabConfig>,
) {
    let window = windows.get_primary_mut().unwrap();
    if keys.just_pressed_many(config.grab_keys) {
        toggle_grab_cursor(window);
    }
}
