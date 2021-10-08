use std::any::Any;

use super::RigDriver;
use bevy::prelude::*;

/// Directly sets the position of the camera
#[derive(Debug)]
pub struct Position {
    pub transform_set: bool,
    pub position: Vec3,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            transform_set: true,
            position: Vec3::ZERO,
        }
    }
}

impl Position {

    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            transform_set: false,
        }
    }

    /// Add the specified vector to the position of this component
    pub fn translate(&mut self, move_vec: Vec3) {
        self.position += move_vec;
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
