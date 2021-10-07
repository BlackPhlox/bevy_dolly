use super::{RigDriver, ExpSmoothed, ExpSmoothingParams};
use bevy::prelude::*;

/// Smooths the parent transformation.
#[derive(Debug)]
pub struct Smooth {
    /// Exponential smoothing factor for the position
    pub position_smoothness: f32,

    /// Exponential smoothing factor for the rotation
    pub rotation_smoothness: f32,

    // The scale with which smoothing should be applied
    output_offset_scale: f32,

    smoothed_position: ExpSmoothed<Vec3>,
    smoothed_rotation: ExpSmoothed<Quat>,
}

impl Default for Smooth {
    fn default() -> Self {
        Self {
            position_smoothness: 1.0,
            rotation_smoothness: 1.0,
            output_offset_scale: 1.0,
            smoothed_position: Default::default(),
            smoothed_rotation: Default::default(),
        }
    }
}

impl Smooth {
    /// Only smooth position
    pub fn new_position(position_smoothness: f32) -> Self {
        Self {
            position_smoothness,
            rotation_smoothness: 0.0,
            ..Default::default()
        }
    }

    /// Only smooth rotation
    pub fn new_rotation(rotation_smoothness: f32) -> Self {
        Self {
            rotation_smoothness,
            position_smoothness: 0.0,
            ..Default::default()
        }
    }

    /// Smooth both position and rotation
    pub fn new_position_rotation(position_smoothness: f32, rotation_smoothness: f32) -> Self {
        Self {
            position_smoothness,
            rotation_smoothness,
            ..Default::default()
        }
    }

    /// Reverse the smoothing, causing the camera to look ahead of the parent transform
    ///
    /// This can be useful on top of [`Position`], and before another `Smooth`
    /// in the chain to create a soft yet responsive follower camera.
    ///
    /// [`Position`]: struct.Position.html
    /// [`Smooth`]: struct.Smooth.html
    pub fn predictive(mut self, predictive: bool) -> Self {
        self.output_offset_scale = if predictive { -1.0 } else { 1.0 };
        self
    }
}

impl RigDriver for Smooth {
    fn update(&mut self, transform: &mut Transform, delta_time_seconds: f32) {
        transform.translation = self.smoothed_position.exp_smooth_towards(
            &transform.translation,
            ExpSmoothingParams {
                smoothness: self.position_smoothness,
                output_offset_scale: self.output_offset_scale,
                delta_time_seconds: delta_time_seconds,
            },
        );

        transform.rotation = self.smoothed_rotation.exp_smooth_towards(
            &transform.rotation,
            ExpSmoothingParams {
                smoothness: self.rotation_smoothness,
                output_offset_scale: self.output_offset_scale,
                delta_time_seconds: delta_time_seconds,
            },
        );
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
