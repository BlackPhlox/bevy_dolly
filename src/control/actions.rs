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
    // TODO: test different structure, this feels heavy
    pub key_map: StableHashMap<Action, KeyCode>,
    pub mouse_map: StableHashMap<Action, MouseButton>,
}

impl Default for ControlActions {
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
        mouse.insert(Action::EnableLook,MouseButton::Right);
        mouse.insert(Action::Up,MouseButton::Other(1));

        Self { key_map: keys, mouse_map: mouse }
    }
}

impl ControlActions {
    /// Helpers function to see if any of the keys are pressed
    pub fn key_pressed(&self, action: Action, input: &Input<KeyCode>) -> bool {
        match self.key_map.get(&action) {
            Some(key) => {
                if input.pressed (*key) {
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
                if input.pressed (*button) {
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
