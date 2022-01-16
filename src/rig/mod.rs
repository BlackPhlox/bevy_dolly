pub mod driver;
use bevy::prelude::*;
pub use driver::*;

/// A chain of drivers, calculating displacements, and animating in succession.
#[derive(Component)]
pub struct Rig {
    /// Drivers to excute in order
    pub drivers: Vec<Box<dyn RigDriver>>,

    /// Will be set when added
    pub target: Transform,

    // smoothness
    pub position_smoothness: f32,
    pub rotation_smoothness: f32,
}

impl Default for Rig {
    fn default() -> Self {
        Self {
            target: Transform::identity(),
            drivers: vec![],
            position_smoothness: 1.0,
            rotation_smoothness: 1.0,
        }
    }
}

impl Rig {

    /// Returns the first driver of the matching type
    pub fn get_driver_mut<T: RigDriver>(&mut self) -> Option<&mut T> {
        for driver in self.drivers.iter_mut() {
            if let Some(a) = driver.as_any_mut().downcast_mut::<T>() {
                return Some(a);
            }
        }
        None
    }

    /// Runs all the drivers in sequence, animating the rig, and producing a final transform of the camera.
    /// Camera rigs are approximately framerate independent, so `update` can be called at any frequency.
    pub fn update(&mut self, current: &Transform, delta_time_seconds: f32) -> Transform {
        let mut result = self.target;


        // TODO: This is overly complicated
        let smoothness_multi: f32 = 8.0;
        let (interp_pos, interp_rot) = (
            1.0 - (-smoothness_multi * delta_time_seconds / self.position_smoothness.max(1e-5))
                .exp(),
            1.0 - (-smoothness_multi * delta_time_seconds / self.rotation_smoothness.max(1e-5))
                .exp(),
        );
        result.translation = Vec3::lerp(current.translation, self.target.translation, interp_pos);
        result.rotation = Quat::lerp(current.rotation, self.target.rotation, interp_rot);

        // excute drivers in order
        for driver in self.drivers.iter_mut() {
            driver.update(&mut result, delta_time_seconds);
        }
        result
    }

    /// Add driver in order to rig
    pub fn with(mut self, driver: impl RigDriver) -> Self {
        self.drivers.push(Box::new(driver));
        self
    }
}
