use super::RigDriver;
use bevy::prelude::*;
use std::any::Any;

/// Directly sets the position of the camera
#[derive(Debug)]
pub struct Position {
    pub (crate) init_set: bool,
    pub position: Vec3,
}

impl Default for Position {
    /// Will use the transform for init value
    fn default() -> Self {
        Self {
            init_set: true,
            position: Vec3::ZERO,
        }
    }
}

impl Position {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            init_set: false,
        }
    }
}

impl RigDriver for Position {
    fn update(&mut self, transform: &mut Transform, _delta_time_seconds: f32) {
        transform.translation = self.position.clone();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
