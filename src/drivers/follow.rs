use dolly::{
    driver::RigDriver,
    glam::Vec3,
    prelude::{Arm, CameraRig, LookAt, Position, Rotation, Smooth},
};

#[derive(Debug)]
pub struct Follow(CameraRig);

impl RigDriver for Follow {
    fn update(&mut self, params: dolly::rig::RigUpdateParams) -> dolly::transform::Transform {
        self.0.update(params.delta_time_seconds)
    }
}

impl Follow {
    pub fn init(transform: dolly::transform::Transform) -> Self {
        Self(
            CameraRig::builder()
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
        )
    }

    pub fn follow(&mut self, position: Vec3, rotation: dolly::glam::Quat, target: Vec3) {
        self.0.driver_mut::<Position>().position = position;
        self.0.driver_mut::<Rotation>().rotation = rotation;
        self.0.driver_mut::<LookAt>().target = target;
    }
}
