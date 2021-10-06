use crate::{RigDriver, RigUpdateParams};
use bevy::prelude::*;

/// Directly sets the position of the camera
#[derive(Default, Debug)]
pub struct Position {
    pub position: Vec3,
}

impl Position {
    ///
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }

    /// Add the specified vector to the position of this component
    pub fn translate(&mut self, move_vec: Vec3) {
        self.position += move_vec;
    }
}

impl RigDriver for Position {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            translation: self.position,
            rotation: params.parent.rotation,
            ..Default::default()
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
