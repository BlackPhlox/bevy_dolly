use bevy::prelude::{Component, Deref, DerefMut};
use dolly::{
    driver::RigDriverTraits,
    prelude::{CameraRig, RightHanded},
};

#[derive(Component, Deref, DerefMut)]
pub struct CR(CameraRig<RightHanded>);

impl CR {
    /// Use this to make a new rig
    pub fn builder() -> CRB {
        CRB {
            drivers: Default::default(),
        }
    }
}

//#[derive(Deref, DerefMut)]
pub struct CRB {
    drivers: Vec<Box<dyn RigDriverTraits<RightHanded>>>
}

impl CRB {
    pub fn with(mut self, driver: impl RigDriverTraits<RightHanded>) -> Self {
        self.drivers.push(Box::new(driver));
        self
    }

    ///
    pub fn build(self) -> CR {
        let mut rig = CameraRig::builder();
        for driver in self.drivers {
            rig.with(driver);
        }
        CR(rig.build())
    }
}

/*

impl CR {
    /// Use this to make a new rig
    pub fn builder() -> CameraRigBuilder<RightHanded> {
        CameraRigBuilder {
            drivers: Default::default(),
            phantom: PhantomData,
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct CRB(CameraRigBuilder<RightHanded>);

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
            phantom: PhantomData, <-- Phantom is a private field
        });

        // Update once to find the final transform
        rig.update(0.0);
        rig
    }
}

*/
