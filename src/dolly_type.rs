use crate::dolly::prelude::*;
use bevy::prelude::{Component, Deref, DerefMut};

#[derive(Component, Deref, DerefMut)]
pub struct Rig(CameraRig);

impl Rig {
    /// Use this to make a new rig
    pub fn builder() -> RigBuilder {
        RigBuilder(CameraRig::builder())
    }
}

pub struct RigBuilder(CameraRigBuilder);

impl RigBuilder {
    pub fn with(mut self, driver: impl RigDriverTraits) -> Self {
        let dolly_crb = self.0.with(driver);
        self.0 = dolly_crb;
        self
    }

    pub fn build(self) -> Rig {
        Rig(self.0.build())
    }
}
