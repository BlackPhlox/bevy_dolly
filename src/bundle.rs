use bevy::{
    prelude::*,
    render::{camera::*, render_graph::base},
    utils::HashMap,
};

use crate::{drivers::*, Rig};

#[derive(Bundle)]
pub struct DollyCameraBundle {
    //pub rig_builder: RigBuilder,
    pub rig: Rig,

    pub camera: Camera,
    pub perspective_projection: PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

/// These are DollyCamera with and additional CameraActions
/// and 1 system will process actions for t hem
#[derive(Bundle)]
pub struct DollyControlCameraBundle {
    /// Hashmap of actions to Vec<KeyCode> that we listen for
    pub camera_actions: CameraActions,
    pub rig: Rig,

    // TODO: Check out #[bundle] again that I understand things better
    // Camera stuff we steal
    pub camera: Camera,
    pub perspective_projection: PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for DollyCameraBundle {
    fn default() -> Self {
        Self {
            rig: Rig::default(),
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


impl Default for DollyControlCameraBundle {
    fn default() -> Self {
        
        Self {
            camera_actions: CameraActions::default(),
            rig: Rig::default()
                .add(Position::default())
                .add(Rotation::default())
                .add(YawPitch::default())
                .add(Smooth::new_position_rotation(1.0, 1.0)),
            //rig: Rig::default(),
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

impl DollyControlCameraBundle {
    // Provide few easy use default cameras
    pub fn new(preset: ControlledType) -> Self {
        info!("here");



        let result = match preset {
            ControlledType::Free => Self {
                rig: Rig::default()
                    .add(Position::default())
                    .add(Rotation::default())
                    .add(YawPitch::default())
                    .add(Smooth::new_position_rotation(1.0, 1.0)),
                camera_actions: CameraActions::default(),
                //rig: Rig::default(),
                camera: Camera {
                    name: Some(base::camera::CAMERA_3D.to_string()),
                    ..Default::default()
                },
                perspective_projection: Default::default(),
                visible_entities: Default::default(),
                transform: Default::default(),
                global_transform: Default::default(),
            },
            ControlledType::FPS => Self {
                rig: Rig::default()
                    .add(Position::default())
                    .add(Rotation::default())
                    .add(YawPitch::default())
                    .add(Smooth::new_position_rotation(1.0, 1.0)),
                camera_actions: CameraActions::default(),
                //rig: Rig::default(),
                camera: Camera {
                    name: Some(base::camera::CAMERA_3D.to_string()),
                    ..Default::default()
                },
                perspective_projection: Default::default(),
                visible_entities: Default::default(),
                transform: Default::default(),
                global_transform: Default::default(),
            },
        };
        result
    }
}

#[derive(Default, Debug)]
pub enum ControlledType {
    #[default]
    Free,
    FPS,
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
pub struct CameraActions {
    pub map: HashMap<CameraAction, Vec<KeyCode>>,
}

impl Default for CameraActions {
    fn default() -> Self {

        let mut keys: HashMap<CameraAction, Vec<KeyCode>> = HashMap::default();

        keys.insert(CameraAction::Forward, vec![KeyCode::Up, KeyCode::W]);
        keys.insert(CameraAction::Backward, vec![KeyCode::Down, KeyCode::S]);
        keys.insert(CameraAction::Left, vec![KeyCode::Left, KeyCode::A]);
        keys.insert(CameraAction::Right, vec![KeyCode::Right, KeyCode::D]);
        keys.insert(CameraAction::Up, vec![KeyCode::Z]);
        keys.insert(CameraAction::Down, vec![KeyCode::X]);
        keys.insert(CameraAction::RotateLeft, vec![KeyCode::Q]);
        keys.insert(CameraAction::RotateRight, vec![KeyCode::E]);
        keys.insert(CameraAction::Boost, vec![KeyCode::LShift]);

        Self { map: keys }
    }
}

impl CameraActions {
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
            }
            None => false,
        }
    }
}
