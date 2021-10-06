use crate::{RigDriver, RigUpdateParams};
use bevy::prelude::*;

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
            ..Default::default()
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
