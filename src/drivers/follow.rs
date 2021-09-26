use dolly::{
    driver::RigDriver,
    glam::Vec3,
    prelude::{Arm, CameraRig, LookAt, Position, Rotation, Smooth},
    rig::RigUpdateParams,
    DollyDriver,
};

#[derive(Debug, DollyDriver)]
pub struct Follow {
    pub rig: CameraRig,
}

impl Follow {
    pub fn init(transform: dolly::transform::Transform) -> Self {
        Self {
            rig: CameraRig::builder()
                .with(Position::new(transform.position))
                .with(Rotation::new(transform.rotation))
                .with(Smooth::new_position(1.25).predictive(true))
                .with(Arm::new(Vec3::new(0.0, 1.5, -3.5)))
                .with(Smooth::new_position(2.5))
                .with(
                    LookAt::new(transform.position + Vec3::Y)
                        .tracking_smoothness(1.25)
                        .tracking_predictive(true),
                )
                .build(),
        }
    }

    pub fn update(&mut self, position: Vec3, rotation: dolly::glam::Quat, target: Vec3) {
        self.rig.driver_mut::<Position>().position = position;
        self.rig.driver_mut::<Rotation>().rotation = rotation;
        self.rig.driver_mut::<LookAt>().target = target;
    }
}
