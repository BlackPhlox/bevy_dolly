use std::marker::PhantomData;

use bevy::prelude::{Component, Deref, DerefMut};
use dolly::{
    driver::RigDriverTraits,
    prelude::{CameraRig, Handedness, RightHanded},
    transform::Transform,
};

#[derive(Component, Deref, DerefMut)]
pub struct CR(CameraRig<RightHanded>);

impl CR {
    /// Use this to make a new rig
    pub fn builder() -> CameraRigBuilder<RightHanded> {
        CameraRigBuilder {
            drivers: Default::default(),
            phantom: PhantomData,
        }
    }
}

pub struct CameraRigBuilder<H: Handedness> {
    drivers: Vec<Box<dyn RigDriverTraits<H>>>,
    phantom: PhantomData<H>,
}

impl CameraRigBuilder<RightHanded> {
    ///
    pub fn with(mut self, driver: impl RigDriverTraits<RightHanded>) -> Self {
        self.drivers.push(Box::new(driver));
        self
    }

    ///
    pub fn build(self) -> CR {
        let mut rig = CR(CameraRig {
            drivers: self.drivers,
            // Initialize with a dummy identity transform. Will be overridden in a moment.
            final_transform: Transform::IDENTITY,
            phantom: PhantomData,
        });

        // Update once to find the final transform
        rig.update(0.0);
        rig
    }
}
