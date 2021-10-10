use bevy::{prelude::*, utils::{StableHashMap, AHashExt}};

// NOTE: Most actions are not implented for both
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
pub struct ControlActions {
    pub key_map: StableHashMap<Action, Vec<KeyCode>>,
    pub mouse_map: StableHashMap<Action, MouseButton>,
}

impl Default for ControlActions {
    fn default() -> Self {

        let mut keys: StableHashMap<Action, Vec<KeyCode>> = StableHashMap::with_capacity(9);
        keys.insert(Action::Forward, vec![KeyCode::Up, KeyCode::W]);
        keys.insert(Action::Backward, vec![KeyCode::Down, KeyCode::S]);
        keys.insert(Action::Left, vec![KeyCode::Left, KeyCode::A]);
        keys.insert(Action::Right, vec![KeyCode::Right, KeyCode::D]);
        keys.insert(Action::Up, vec![KeyCode::Z]);
        keys.insert(Action::Down, vec![KeyCode::X]);
        keys.insert(Action::RotateLeft, vec![KeyCode::Q]);
        keys.insert(Action::RotateRight, vec![KeyCode::E]);
        keys.insert(Action::Boost, vec![KeyCode::LShift]);

        let mut mouse: StableHashMap<Action, MouseButton> = StableHashMap::with_capacity(1);
        mouse.insert(Action::EnableLook,MouseButton::Right);
    
        Self { key_map: keys, mouse_map: mouse }
    }
}

impl ControlActions {
    /// Helpers function to see if any of the keys are pressed
    pub fn key_pressed(&self, action: Action, input: &Input<KeyCode>) -> bool {
        match self.key_map.get(&action) {
            Some(keys) => {
                // TODO: try to get any_pressed working
                // input.any_pressed( keys ) without coping vec
                for key in keys.iter() {
                    if input.pressed(*key) {
                        return true;
                    }
                }
                false
            }
            None => false,
        }
    }

    pub fn mouse_pressed(&self, action: Action, input: &Input<MouseButton>) -> bool {
        match self.mouse_map.get(&action) {
            Some(button) => {
                if input.pressed (*button) {
                    return true;
                }
                false
            }
            None => false,
        }
    }

    pub fn print_actions(&self) {
        info!("Camera Actions - {} keys, {} mouse buttons", self.key_map.len(), self.mouse_map.len());
        for (action, keys) in self.key_map.iter() {
            let key_text  = keys.iter().map( |k| format!("{:?} ", k)).collect::<String>();
            info!(" action: {:?}, keys: {}", action, key_text);
        }
        for (action, btn) in self.mouse_map.iter() {
            info!(" action: {:?}, button: {:?}", action, btn);
        }
    }
}
