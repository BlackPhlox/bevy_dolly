mod actions;
mod bundle;
pub use actions::*;
use bevy::{input::mouse::MouseMotion, prelude::*};
pub use bundle::*;

use crate::rig::Rig;

/// Configuration Resource for Dolly Controlled Rigs
// TODO: We could store the targeting data here, would really make user
// interaction
pub struct DollyControlConfig {
    pub speed: f32,
    pub key_rotation: f32,
    pub boost_multiplyer: f32,
    pub sensitivity: Vec2,
}

impl Default for DollyControlConfig {
    fn default() -> Self {
        Self {
            speed: 10.0,
            key_rotation: 15.0,
            boost_multiplyer: 5.0,
            sensitivity: Vec2::splat(0.001),
        }
    }
}

/// Updates rigs with a generic control system
///
/// This only runs for DollyControlCameraBundles, not DollyCameraBundles
pub fn update_control_system(
    time: Res<Time>,
    input_keys: Res<Input<KeyCode>>,
    input_mouse_btn: Res<Input<MouseButton>>,
    config: Res<DollyControlConfig>,
    mut windows: ResMut<Windows>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&Transform, &mut Rig, &ControlActions)>,
) {
    for (t, mut rig, control_actions) in query.iter_mut() {
        let window = windows.get_primary_mut().unwrap();
        // Update position
        let mut move_vec = Vec3::ZERO;
        if control_actions.key_pressed(Action::Forward, &input_keys) {
            move_vec.z -= 1.0;
        }
        if control_actions.key_pressed(Action::Backward, &input_keys) {
            move_vec.z += 1.0;
        }
        if control_actions.key_pressed(Action::Left, &input_keys) {
            move_vec.x -= 1.0;
        }
        if control_actions.key_pressed(Action::Right, &input_keys) {
            move_vec.x += 1.0;
        }
        if control_actions.key_pressed(Action::Up, &input_keys) {
            move_vec.y += 1.0;
        }
        if control_actions.key_pressed(Action::Down, &input_keys) {
            move_vec.y -= 1.0;
        }

        // apply a turbo
        let boost = match control_actions.key_pressed(Action::Boost, &input_keys) {
            true => config.boost_multiplyer,
            false => 1.0,
        };

        // Make movement relative to current transform(camera) and limit effect
        move_vec = t.rotation * move_vec.clamp_length_max(1.0);
        //move_vec.y = 0.0;

        // Apply the move
        rig.target.translation += move_vec * time.delta_seconds() * config.speed * boost;

        // Update rotation
        let mut delta = Vec2::ZERO;

        if control_actions.key_pressed(Action::RotateLeft, &input_keys) {
            delta.x -= 10.0;
        }
        if control_actions.key_pressed(Action::RotateRight, &input_keys) {
            delta.x += 10.0;
        }

        // Mouse Enable Look
        if let Some(btn) = control_actions.mouse_map.get(&Action::EnableLook) {
            look_around(
                window,
                &input_mouse_btn,
                btn,
                &mut mouse_motion_events,
                &mut delta,
            );
        }
        if let Some(key) = control_actions.key_map.get(&Action::EnableLook) {
            look_around(
                window,
                &input_keys,
                key,
                &mut mouse_motion_events,
                &mut delta,
            );
        }

        // Apply rotation
        rig.target
            .rotate(Quat::from_rotation_x(-config.sensitivity.y * delta.y));
        rig.target
            .rotate(Quat::from_rotation_y(-config.sensitivity.x * delta.x));
    }
}

fn look_around<T: Copy + Eq + std::hash::Hash>(
    window: &mut Window,
    input: &Input<T>,
    btn: &T,
    mouse_motion_events: &mut EventReader<MouseMotion>,
    delta: &mut Vec2,
) {
    if input.just_pressed(*btn) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }
    if input.just_released(*btn) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
    if input.pressed(*btn) {
        for event in mouse_motion_events.iter() {
            *delta += event.delta;
        }
    }
}
