use bevy::prelude::{Deref, DerefMut, Quat, Transform, Vec3};
use dolly::{driver::RigDriver, prelude::*};

impl MovableLookAt {
    pub fn from_position_target(target_position: Vec3) -> Self {
        Self(
            CameraRig::builder()
                .with(Position::new(target_position))
                .with(Rotation::new(Quat::IDENTITY))
                .with(Smooth::new_position(1.25).predictive(true))
                .with(Arm::new(Vec3::new(0.0, 1.5, -3.5)))
                .with(Smooth::new_position(2.5))
                .with(
                    LookAt::new(target_position + Vec3::Y)
                        .tracking_smoothness(1.25)
                        .tracking_predictive(true),
                )
                .build(),
        )
    }

    pub fn set_position_target(&mut self, target_position: Vec3, target_rotation: Quat) {
        self.driver_mut::<Position>().position = target_position;
        self.driver_mut::<Rotation>().rotation = target_rotation;
        self.driver_mut::<LookAt>().target = target_position + Vec3::Y;
    }
}

/// A custom camera rig which combines smoothed movement with a look-at driver.
#[derive(Debug, Deref, DerefMut)]
pub struct MovableLookAt(CameraRig);

// Turn the nested rig into a driver, so it can be used in another rig.
impl RigDriver for MovableLookAt {
    fn update(&mut self, params: dolly::rig::RigUpdateParams) -> Transform {
        self.0.update(params.delta_time_seconds)
    }
}
