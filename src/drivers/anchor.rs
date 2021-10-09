use super::RigDriver;
use bevy::prelude::*;
use std::any::Any;

/// Sets Transform to Target's
/// Should only be used first in a Rig
#[derive(Debug)]
pub struct Anchor {
    /// The world-space position to look at
    pub target_entity: Entity,

    /// The world-space position to look at
    pub target: Transform,
}

impl Anchor {
    /// Anchor to entity transform
    pub fn new(target: Entity) -> Self {
        Self {
            target_entity: target,
            target: Transform::default(),
        }
    }
}

impl RigDriver for Anchor {
    fn update(&mut self, transform: &mut Transform, _delta_time_seconds: f32) {
        *transform = self.target;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
