use std::any::Any;

use super::RigDriver;
use bevy::prelude::*;

/// Smooths the parent transformation.
#[derive(Debug)]
pub struct Smooth {
    /// Exponential smoothing factor for the position
    pub position_smoothness: f32,
    pub rotation_smoothness: f32,

    prev_position: Option<Vec3>,
    prev_rotation: Option<Quat>,
}

impl Default for Smooth {
    fn default() -> Self {
        Self {
            position_smoothness: 1.0,
            rotation_smoothness: 1.0,
            prev_position: None,
            prev_rotation: None,
        }
    }
}

impl Smooth {
    pub fn new(position: f32, rotation: f32) -> Self {
        Self {
            position_smoothness: position,
            rotation_smoothness: rotation,
            prev_position: None,
            prev_rotation: None,
        }
    }
}



impl RigDriver for Smooth {
    // TODO: This is far clearer, but 2 things
    // - We do all the work no mater if smoothing is enabled
    // - no need 

    /// Smooths translation and/or rotation
    fn update(&mut self, transform: &mut Transform, delta_time_seconds: f32) {
        // Calculate the exponential blending based on frame time


        // An ad-hoc multiplier to make default smoothness parameters
        // produce good-looking results.
        let smoothness_mult: f32 = 8.0;
        let (interp_pos, interp_rot) = (
            1.0 - (-smoothness_mult * delta_time_seconds / self.position_smoothness.max(1e-5))
                .exp(),
            1.0 - (-smoothness_mult * delta_time_seconds / self.rotation_smoothness.max(1e-5))
                .exp(),
        );

        // Find our previous info if any, lerp with target
        let prev_pos = self.prev_position.unwrap_or(transform.translation);
        let prev_rot = self.prev_rotation.unwrap_or(transform.rotation);
        transform.translation = Vec3::lerp(prev_pos, transform.translation, interp_pos);
        transform.rotation = Quat::lerp(prev_rot, transform.rotation, interp_rot);

        // Save
        self.prev_position = Some(transform.translation);
        self.prev_rotation = Some(transform.rotation);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
