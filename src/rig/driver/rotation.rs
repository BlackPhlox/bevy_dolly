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
    pub fn new(yaw_degrees: f32, pitch_degrees: f32) -> Self {
        Self {
            init_set: false,
            rotation: Quat::from_euler(
                bevy::math::EulerRot::YXZ,
                yaw_degrees.to_radians(),
                pitch_degrees.to_radians(),
                0.0,
            ),
        }
    }

    /// Additively rotate by the specified angles.
    pub fn rotate_yaw_pitch(&mut self, yaw: f32, pitch: f32) {
        let yaw_degrees = yaw % 720_f32;
        let pitch_degrees = pitch.clamp(-90.0, 90.0);

        self.rotation  *= Quat::from_euler(
            bevy::math::EulerRot::YXZ,
            yaw_degrees.to_radians(),
            pitch_degrees.to_radians(),
            0.0,
        );
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
