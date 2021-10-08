use super::RigDriver;
use bevy::prelude::*;
use std::any::Any;

/// Sets Transform to Target's
#[derive(Debug)]
pub struct Follow {
    /// The world-space position to look at
    pub target_entity: Entity,

    /// The world-space position to look at
    pub target: Transform,
}

impl Follow {
    ///
    pub fn new(target: Entity) -> Self {
        Self {
            target_entity: target,
            target: Transform::default(),
        }
    }
}

impl RigDriver for Follow {
    fn update(&mut self, transform: &mut Transform, _delta_time_seconds: f32) {
        transform.translation = self.target.translation;
        transform.rotation = self.target.rotation;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
