use std::any::Any;

use super::RigDriver;
use bevy::prelude::*;

/// Directly sets the rotation of the camera
#[derive(Debug)]
pub struct Rotation {
    pub transform_set: bool,
    pub rotation: Quat,
}

impl Default for Rotation {
    /// Will use the transform for init value
    fn default() -> Self {
        Self {
            rotation: Quat::default(),
            transform_set: true,
        }
    }
}

impl Rotation {
    pub fn new(rotation: Quat) -> Self {
        Self {
            rotation,
            transform_set: false,
        }
    }
}

impl RigDriver for Rotation {
    fn update(&mut self, transform: &mut Transform, _delta_time_seconds: f32) {
        transform.rotation = self.rotation;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
