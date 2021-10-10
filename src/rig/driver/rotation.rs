use std::any::Any;

use super::RigDriver;
use bevy::prelude::*;

/// Directly sets the rotation of the camera
#[derive(Debug)]
pub struct Rotation {
    pub rotation: Quat,

    /// Sets using rigs transfrom
    pub(crate) init_set: bool,
}

impl Default for Rotation {
    /// Will use the transform for init value
    fn default() -> Self {
        Self {
            rotation: Quat::default(),
            init_set: true,
        }
    }
}

impl Rotation {
    pub fn new<T: Into<Quat>>(rotation: T) -> Self {
        Self {
            rotation: rotation.into(),
            init_set: false,
        }
    }
}

impl RigDriver for Rotation {
    fn update(&mut self, transform: &mut Transform, _delta_time_seconds: f32) {
        transform.rotation *= self.rotation;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
