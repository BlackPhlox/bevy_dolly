
mod anchor;
mod arm;
mod look_at;
mod smooth;

use bevy::prelude::*;
use std::any::Any;
pub use {anchor::*, arm::*, look_at::*, smooth::*};

pub trait RigDriver: Sync + Send + 'static {
    /// Calculates the transform of this driver component
    fn update(&mut self, transform: &mut Transform, delta_time_seconds: f32);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
