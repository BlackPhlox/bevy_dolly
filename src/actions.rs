use crate::rig::Rig;
use bevy::utils::{AHashExt, StableHashMap};
use bevy::{input::mouse::MouseMotion, math::vec3, prelude::*};

// NOTE: Most actions are not implented for both Keyboard and mouse
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Action {
    // Keyboard
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
    RotateLeft,
    RotateRight,
    Boost,
    ToggleLook,

    // Both possible
    EnableLook,
}

#[derive(Component)]
/// Actions our controller can handle
pub struct DollyActions {
    // TODO: test different structure
    pub key_map: StableHashMap<Action, KeyCode>,
    pub mouse_map: StableHashMap<Action, MouseButton>,
}

impl Default for DollyActions {
    fn default() -> Self {
        let mut keys: StableHashMap<Action, KeyCode> = StableHashMap::with_capacity(10);
        keys.insert(Action::Forward, KeyCode::W);
        keys.insert(Action::Backward, KeyCode::S);
        keys.insert(Action::Left, KeyCode::A);
        keys.insert(Action::Right, KeyCode::D);
        keys.insert(Action::Up, KeyCode::Z);
        keys.insert(Action::Down, KeyCode::X);
        keys.insert(Action::RotateLeft, KeyCode::Q);
        keys.insert(Action::RotateRight, KeyCode::E);
        keys.insert(Action::Boost, KeyCode::LShift);

        keys.insert(Action::EnableLook, KeyCode::Space);

        let mut mouse: StableHashMap<Action, MouseButton> = StableHashMap::with_capacity(2);
        mouse.insert(Action::EnableLook, MouseButton::Right);
        mouse.insert(Action::Up, MouseButton::Other(1));

        Self {
            key_map: keys,
            mouse_map: mouse,
        }
    }
}

impl DollyActions {
    /// Helpers function to see if any of the keys are pressed
    pub fn key_pressed(&self, action: Action, input: &Input<KeyCode>) -> bool {
        match self.key_map.get(&action) {
            Some(key) => {
                if input.pressed(*key) {
                    return true;
                }
                false
            }
            None => false,
        }
    }

    // TODO: remove this fn
    pub fn mouse_pressed(&self, action: Action, input: &Input<MouseButton>) -> bool {
        match self.mouse_map.get(&action) {
            Some(button) => {
                if input.pressed(*button) {
                    return true;
                }
                false
            }
            None => false,
        }
    }

    pub fn print_actions(&self) {
        info!("Key Actions - {}", self.key_map.len());
        for (action, key) in self.key_map.iter() {
            info!(" action: {:?}, key: {:?}", action, key);
        }
        info!("Mouse Actions - {}", self.mouse_map.len());
        for (action, btn) in self.mouse_map.iter() {
            info!(" action: {:?}, button: {:?}", action, btn);
        }
    }
}

/// Configuration Resource for Dolly Controlled Rigs
// TODO: We could store the targeting data here
pub struct DollyControlConfig {
    pub speed: f32,
    pub key_rotation: f32,
    pub boost_multiplyer: f32,
    pub sensitivity: Vec3,
}

impl Default for DollyControlConfig {
    fn default() -> Self {
        Self {
            speed: 10.0,
            key_rotation: 15.0,
            boost_multiplyer: 5.0,
            sensitivity: Vec3::splat(0.001),
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
    mut query: Query<(&Transform, &mut Rig, &DollyActions)>,
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

        // Apply the move
        rig.target.translation += move_vec * time.delta_seconds() * config.speed * boost;

        // Update rotation
        let mut delta = Vec3::ZERO;
        if control_actions.key_pressed(Action::RotateLeft, &input_keys) {
            delta.z += 10.0;
        }
        if control_actions.key_pressed(Action::RotateRight, &input_keys) {
            delta.z -= 10.0;
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

        // TODO: clean this up, and limit rotation to keep level
        rig.target.rotate(Quat::from_euler(
            bevy::math::EulerRot::XYZ,
            config.sensitivity.x * delta.x,
            config.sensitivity.y * delta.y,
            config.sensitivity.z * delta.z,
        ));
    }
}

fn look_around<T: Copy + Eq + std::hash::Hash>(
    window: &mut Window,
    input: &Input<T>,
    btn: &T,
    mouse_motion_events: &mut EventReader<MouseMotion>,
    delta: &mut Vec3,
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
            // TODO: have to reverse this, is there a way in which i don't?
            *delta += vec3(-event.delta.y, -event.delta.x, 0.0);
        }
    }
}
