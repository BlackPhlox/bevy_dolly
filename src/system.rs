use bevy::{input::mouse::MouseMotion, prelude::*};
use dolly::prelude::{CameraRig, Position, Rotation, Smooth, YawPitch};

use crate::{Transform2Bevy, Transform2Dolly, ZeroYRotation};

pub struct MainCamera;

pub struct Cameras {
    pub cameras: Vec<Box<dyn DollyMouseUpdate + Sync + Send + 'static>>,
}

#[allow(clippy::type_complexity)]
pub trait DollyMouseUpdate {
    fn setup_camera(&self, commands: Commands);
    fn update_camera(
        &self,
        time: Res<Time>,
        keys: Res<Input<KeyCode>>,
        windows: Res<Windows>,
        mouse_motion_events: EventReader<MouseMotion>,
        query: Query<(&mut Transform, With<MainCamera>)>,
        query2: Query<&mut CameraRig>,
    );
}

pub struct Fps;
impl DollyMouseUpdate for Fps {
    fn setup_camera(&self, mut commands: Commands) {
        let translation = [-2.0f32, 2.0f32, 5.0f32];
        let transform =
            Transform::from_translation(bevy::math::Vec3::from_slice_unaligned(&translation))
                .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y);

        let rotation = transform.transform_2_dolly().rotation;
        let mut yaw_pitch = YawPitch::new();
        yaw_pitch.set_rotation_quat(rotation);

        commands.spawn().insert(
            CameraRig::builder()
                .with(Position {
                    position: dolly::glam::Vec3::from_slice(&translation),
                })
                .with(Rotation { rotation })
                .with(yaw_pitch)
                .with(Smooth::new_position_rotation(1.0, 0.1))
                .build(),
        );
    }

    #[allow(clippy::type_complexity)]
    fn update_camera(
        &self,
        time: Res<Time>,
        keys: Res<Input<KeyCode>>,
        windows: Res<Windows>,
        mut mouse_motion_events: EventReader<MouseMotion>,
        mut query: Query<(&mut Transform, With<MainCamera>)>,
        mut query2: Query<&mut CameraRig>,
    ) {
        let time_delta_seconds: f32 = time.delta_seconds();
        let boost_mult = 5.0f32;
        let sensitivity = Vec2::splat(1.0);

        let mut move_vec = dolly::glam::Vec3::ZERO;

        // Q: Is dolly left-handed so z is flipped?
        if keys.pressed(KeyCode::W) {
            move_vec.z -= 1.0;
        }
        if keys.pressed(KeyCode::S) {
            move_vec.z += 1.0;
        }
        if keys.pressed(KeyCode::A) {
            move_vec.x -= 1.0;
        }
        if keys.pressed(KeyCode::D) {
            move_vec.x += 1.0;
        }

        if keys.pressed(KeyCode::E) || keys.pressed(KeyCode::Space) {
            move_vec.y += 1.0;
        }
        if keys.pressed(KeyCode::Q) {
            move_vec.y -= 1.0;
        }

        let boost: f32 = if keys.pressed(KeyCode::LShift) {
            1.
        } else {
            0.
        };

        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            delta += event.delta;
        }

        let mut rig = query2.single_mut().unwrap();

        let move_vec = rig.final_transform.rotation.zero_y_rotation()
            * move_vec.clamp_length_max(1.0)
            * boost_mult.powf(boost);

        let window = windows.get_primary().unwrap();
        if window.cursor_locked() {
            rig.driver_mut::<YawPitch>().rotate_yaw_pitch(
                -0.1 * delta.x * sensitivity.x,
                -0.1 * delta.y * sensitivity.y,
            );
            rig.driver_mut::<Position>()
                .translate(move_vec * time_delta_seconds * 10.0);
        }

        let transform = rig.update(time_delta_seconds);
        let (mut cam, _) = query.single_mut().unwrap();

        cam.transform_2_bevy(transform);
    }
}
