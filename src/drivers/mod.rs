mod arm;
mod look_at;
mod position;
mod rotation;
mod smooth;
mod yaw_pitch;

use crate::RigUpdateParams;
use bevy::prelude::*;

pub use {
    arm::*,
    look_at::*,
    position::*,
    rotation::*,
    smooth::*,
    yaw_pitch::*
};

pub trait RigDriver: std::any::Any {
    /// Calculates the transform of this driver component based on the parent
    /// provided in `params`.
    fn update(&mut self, params: RigUpdateParams) -> Transform;

    /// Returns `self` as `&mut dyn Any`
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
