//use glam::Vec3;
use bevy::math::*;

use crate::{driver::RigDriver, rig::RigUpdateParams, transform::Transform};

/// Directly sets the position of the camera
#[derive(Default, Debug)]
pub struct Position {
    pub translation: Vec3,
}

impl Position {
    ///
    pub fn new(translation: Vec3) -> Self {
        Self { translation }
    }

    /// Add the specified vector to the position of this component
    pub fn translate(&mut self, move_vec: Vec3) {
        self.translation += move_vec;
    }
}

impl RigDriver for Position {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            translation: self.translation,
            rotation: params.parent.rotation,
            scale: params.parent.scale,
        }
    }
}
