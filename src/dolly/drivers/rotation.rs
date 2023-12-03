//use std::marker::PhantomData;

//use glam::Quat;
use bevy_math::{Quat, Vec3};
use bevy_transform::prelude::Transform;

use crate::dolly::{driver::RigDriver, rig::RigUpdateParams};

/*
use crate::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform,
};
*/

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
            scale: Vec3::ONE,
        }
    }
}
