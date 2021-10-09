use std::any::Any;

use super::RigDriver;
use bevy::math::*;
use bevy::prelude::*;

/// Calculate camera rotation based on yaw and pitch angles.
///
/// The angles follow the [`right-hand rule`] for curve orientation, and assume
/// an `OpenGL`-style coordinate system, meaning that for a camera to rotate right,
/// a negative value of yaw should be provided, and for it to rotate up,
/// a positive value of pitch.
///
/// [`right-hand rule`]: https://en.wikipedia.org/wiki/Right-hand_rule#Curve_orientation_and_normal_vectors
#[derive(Default, Debug)]
pub struct YawPitch {
    /// [0..720)
    ///
    /// Note: Quaternions can encode 720 degrees of rotation, causing a slerp from 350 to 0 degrees
    /// to happen counter-intuitively in the negative direction; the positive direction would go through 720,
    /// thus being farther. By encoding rotation here in the 0..720 range, we reduce the risk of this happening.
    pub yaw_degrees: f32,

    /// [-90..90]
    pub pitch_degrees: f32,
}

impl YawPitch {
    pub fn new(yaw_degrees: f32, pitch_degrees: f32) -> Self {
        Self {
            yaw_degrees,
            pitch_degrees,
        }
    }   
    /// Additively rotate by the specified angles.
    pub fn rotate_yaw_pitch(&mut self, yaw_degrees: f32, pitch_degrees: f32) {
        self.yaw_degrees = (self.yaw_degrees + yaw_degrees) % 720_f32;
        self.pitch_degrees = (self.pitch_degrees + pitch_degrees).clamp(-90.0, 90.0);
    }
}

impl RigDriver for YawPitch {
    fn update(&mut self, transform: &mut Transform, _delta_time_seconds: f32) {
        transform.rotation = Quat::from_euler(
            bevy::math::EulerRot::YXZ,
            self.yaw_degrees.to_radians(),
            self.pitch_degrees.to_radians(),
            0.0,
        );
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
