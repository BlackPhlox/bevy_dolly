//use glam::Quat;

use bevy::math::*;

use crate::{driver::RigDriver, rig::RigUpdateParams, transform::Transform};

/// Directly sets the rotation of the camera
#[derive(Default, Debug)]
pub struct Rotation {
    pub rotation: Quat,
}

impl Rotation {
    pub fn new(rotation: Quat) -> Self {
        Self { rotation }
    }
}

impl RigDriver for Rotation {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            translation: params.parent.translation,
            rotation: self.rotation,
            scale: params.parent.scale,
        }
    }
}
