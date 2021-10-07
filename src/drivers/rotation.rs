use std::any::Any;

use super::{RigDriver};
use bevy::prelude::*;

/// Directly sets the rotation of the camera
#[derive(Default, Debug)]
pub struct Rotation {
    pub rotation: Quat,
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
