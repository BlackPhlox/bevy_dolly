use bevy::{prelude::*, render::{camera::{Camera, PerspectiveProjection, VisibleEntities}, render_graph::base}};
use crate::rig::{Rig};

use super::actions::*;

/// These are DollyCamera with and additional CameraActions
/// and 1 system will process actions for them
#[derive(Bundle)]
pub struct DollyControlCameraBundle {
    pub control_actions: ControlActions,
    pub rig: Rig,

    // TODO: Check out #[bundle] again that I understand things better
    // Camera stuff we steal
    pub camera: Camera,
    pub perspective_projection: PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for DollyControlCameraBundle {
    fn default() -> Self {
        Self {
            control_actions: ControlActions::default(),
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

impl DollyControlCameraBundle {
    /// Provide few easy use default cameras
    /// TODO: Flush this out more with tested presets
    pub fn new(preset: ControlledType) -> Self {
       match preset {
            // WARN: Don't call default on control camera bundle its self, you will get a loop
            ControlledType::Free => Self {
                rig: Rig::default(),
                control_actions: ControlActions::default(),

                camera: Camera {
                    name: Some(base::camera::CAMERA_3D.to_string()),
                    ..Default::default()
                },
                perspective_projection: Default::default(),
                visible_entities: Default::default(),
                transform: Default::default(),
                global_transform: Default::default(),
            },
            ControlledType::Fps => Self {
                rig: Rig::default(),
                control_actions: ControlActions::default(),
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
        }
    }
}

#[derive(Default, Debug)]
pub enum ControlledType {
    #[default]
    Free,
    Fps,
}