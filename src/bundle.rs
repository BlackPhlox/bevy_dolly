use crate::rig::Rig;
use bevy::{
    prelude::*,
    render::{camera::*, render_graph::base},
};

#[derive(Bundle)]
pub struct DollyCameraBundle {
    pub rig: Rig,
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
