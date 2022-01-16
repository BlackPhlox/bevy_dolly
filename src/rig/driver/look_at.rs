use super::RigDriver;
use bevy::prelude::*;
use std::any::Any;

/// Rotates the camera to point at a world-space position.
///
/// The target tracking can be additionally smoothed, anad made to look ahead of it.
#[derive(Debug)]
pub struct LookAt {
    /// The world-space position to look at
    pub target_entity: Option<Entity>,
    pub target_transform: Option<Transform>,

    /// An Offset from target perspective
    pub offset: Vec3,
}

impl LookAt {
    /// Create a target to look at, you can provide and offset
    /// from the targets perspective
    pub fn new(target: Entity, offset: Vec3) -> Self {
        Self {
            target_entity: Some(target),
            offset,
            target_transform: None,
        }
    }
}

impl RigDriver for LookAt {
    fn update(&mut self, transform: &mut Transform, _delta_time_seconds: f32) {
        let mut world_target = Vec3::ZERO;
        if let Some(t) = self.target_transform {
            world_target = t.translation + (t.rotation * self.offset);
        }
        transform.look_at(world_target, Vec3::Y);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
