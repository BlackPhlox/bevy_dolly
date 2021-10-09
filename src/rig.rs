use crate::drivers::*;
use bevy::prelude::*;

/// A chain of drivers, calculating displacements, and animating in succession.
#[derive(Default, Component)]
pub struct Rig {
    /// Drivers to excute in order
    pub drivers: Vec<Box<dyn RigDriver>>,
}

impl Rig {
    /// Returns the first driver of the matching type
    // TODO: any cleaner way? old way would panic, screw that
    pub fn get_driver_mut<T: RigDriver>(&mut self) -> Option<&mut T> {
        for driver in self.drivers.iter_mut() {
            match driver.as_any_mut().downcast_mut::<T>() {
                Some(a) => return Some(a),
                None => (),
            }
        }
        None
    }

    /// Runs all the drivers in sequence, animating the rig, and producing a final transform of the camera.
    /// Camera rigs are approximately framerate independent, so `update` can be called at any frequency.
    pub fn update(&mut self, delta_time_seconds: f32) -> Transform {

        // You would think that we would start with our current
        // transform or some thing we saved, but it limit driver capabilities
        // ex. (rotation, position) for a free camara vs follow driver
        // TODO: Transform have scale, that we never ever use, should test other options
        let mut result = Transform::default();
        // excute drivers in order
        for driver in self.drivers.iter_mut() {
            driver.update(&mut result, delta_time_seconds);
        }
        result
    }

    /// Add driver in order to rig
    pub fn add(mut self, driver: impl RigDriver) -> Self {
        self.drivers.push(Box::new(driver));
        self
    }
}
