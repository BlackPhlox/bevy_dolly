use crate::drivers::*;
use bevy::prelude::*;

/// A chain of drivers, calculating displacements, and animating in succession.
#[derive(Component, Default)]
pub struct CameraRig {
    pub (crate) drivers: Vec<Box<dyn RigDriver>>,
    pub final_transform: Transform,
}

impl CameraRig {

    /// Returns the first driver of the matching type. Panics if no such driver is present.
    pub fn driver_mut<T: RigDriver>(&mut self) -> Option<&mut T> {
        // TODO: Look for clearer type check
        self.drivers
            .iter_mut()
            .find_map(|driver| {
                driver.as_mut().as_any_mut().downcast_mut::<T>()
            })
    }

    pub fn with(mut self, driver: impl RigDriver) -> Self {
        self.drivers.push(Box::new(driver));
        self
    }

    /// Runs all the drivers in sequence, animating the rig, and producing a final transform of the camera.
    ///
    /// Camera rigs are approximately framerate independent, so `update` can be called at any frequency.
    pub fn process(&mut self,  current: &mut Transform, delta_time_seconds: f32) -> Transform {
        let mut parent_transform = Transform::default();

        for driver in self.drivers.iter_mut() {
            driver.update(&mut parent_transform, delta_time_seconds);
        }

        self.final_transform = parent_transform;
        self.final_transform
    }

}

