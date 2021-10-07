use bevy::{prelude::*, render::{camera::*, render_graph::base}, utils::HashMap};

use crate::{drivers::*, CameraRig};

#[derive(Bundle)]
pub struct DollyCameraBundle {
    pub camera_rig: CameraRig,
    pub camera_keys: CameraActionMap,
    pub camera: Camera,
    pub perspective_projection: PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for DollyCameraBundle {
    fn default() -> Self {
        Self {
            camera_rig: CameraRig::default()
                 .with(Smooth::new_position_rotation(1.0, 1.0)),
            camera_keys: CameraActionMap::default(),
            camera: Camera {
                name: Some(base::camera::CAMERA_3D.to_string()),
                ..Default::default()
            },
            perspective_projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

impl DollyCameraBundle {
    pub fn new_fps() -> Self {
        Self {
            // camera_keys: CameraActionMap::new_fps(),
            // camera_rig: CameraRig::default()
            //     .with(Smooth::new_position_rotation(1.0, 1.0)),
            camera: Camera {
                name: Some(base::camera::CAMERA_3D.to_string()),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}


#[derive(PartialEq, Eq, Hash)]
pub enum CameraAction {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
    RotateLeft,
    RotateRight,
    Boost,
}


#[derive(Component)]
/// Hashmap of actions and keycodes
pub struct CameraActionMap {
    pub map: HashMap<CameraAction, Vec<KeyCode>>,
}

impl Default for CameraActionMap {
    fn default() -> Self {
        let mut keys: HashMap<CameraAction, Vec<KeyCode>> = HashMap::default();

        keys.insert(CameraAction::Forward, vec![KeyCode::Up, KeyCode::W]);
        keys.insert(CameraAction::Backward, vec![KeyCode::Down, KeyCode::S]);
        keys.insert(CameraAction::Left, vec![KeyCode::Comma, KeyCode::A]);
        keys.insert(CameraAction::Right, vec![KeyCode::Period, KeyCode::D]);
        keys.insert(CameraAction::Up, vec![KeyCode::RShift]);
        keys.insert(CameraAction::Down, vec![KeyCode::Minus]);
        keys.insert(CameraAction::RotateLeft, vec![KeyCode::Left]);
        keys.insert(CameraAction::RotateRight, vec![KeyCode::Right]);
        keys.insert(CameraAction::Boost, vec![KeyCode::LShift]);

        Self { map: keys }
    }
}

impl CameraActionMap {

    pub fn pressed(&self, action: CameraAction, input: &Input<KeyCode>) -> bool {
        match self.map.get(&action) {
            Some(keys) => {
                // TODO: try to get any_pressed working
                // input.any_pressed( keys ) without copy vec
                for key in keys.iter() {
                    if input.pressed(*key) {
                        return true;
                    }
                }
                false
            },
            None => false,
        }
    }

    pub fn new_fps() -> Self {
        let mut keys: HashMap<CameraAction, Vec<KeyCode>> = HashMap::default();

        keys.insert(CameraAction::Forward, vec![KeyCode::Up, KeyCode::W]);
        keys.insert(CameraAction::Backward, vec![KeyCode::Down, KeyCode::S]);
        keys.insert(CameraAction::Left, vec![KeyCode::Left, KeyCode::A]);
        keys.insert(CameraAction::Right, vec![KeyCode::Right, KeyCode::D]);
        keys.insert(CameraAction::RotateLeft, vec![KeyCode::Q]);
        keys.insert(CameraAction::RotateRight, vec![KeyCode::R]);

        Self { map: keys }
    }
}
