use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use dolly::driver::RigDriver;
use dolly::glam::Vec3;
use dolly::prelude::{CameraRig, Position, Rotation, Smooth, YawPitch};
use dolly::rig::RigUpdateParams;
use std::fmt::Debug;

use crate::{IterAnyPressed, WithRigSettings, ZeroYRotation};

#[derive(Debug)]
pub struct Fps {
    pub rig: CameraRig,
}

pub struct Vec3KeyMapWithBoost {
    pub forward: &'static [KeyCode],
    pub backward: &'static [KeyCode],
    pub left: &'static [KeyCode],
    pub right: &'static [KeyCode],
    pub up: &'static [KeyCode],
    pub down: &'static [KeyCode],
    pub boost: &'static [KeyCode],
}

impl Default for Vec3KeyMapWithBoost {
    fn default() -> Self {
        Self {
            forward: &[KeyCode::Up, KeyCode::W],
            backward: &[KeyCode::Down, KeyCode::S],
            left: &[KeyCode::A],
            right: &[KeyCode::D],
            up: &[KeyCode::RShift, KeyCode::Q, KeyCode::Space],
            down: &[KeyCode::Minus, KeyCode::LControl, KeyCode::E],
            boost: &[KeyCode::LShift],
        }
    }
}

impl Fps {
    pub fn update(
        &mut self,
        time: Res<Time>,
        keys: Res<Input<KeyCode>>,
        windows: Res<bevy::window::Windows>,
        mut mouse_motion_events: EventReader<MouseMotion>,
        sensitivity: Vec2,
        map: Vec3KeyMapWithBoost,
    ) {
        let time_delta_seconds: f32 = time.delta_seconds();
        let mut move_vec = Vec3::ZERO;
        let mut delta = Vec2::ZERO;
        let boost_mult = 5.0f32;
        let mut boost = 0.0;

        // Q: Is dolly left-handed so z is flipped?
        for key in keys.get_pressed() {
            if map.forward.is_being_pressed(key) {
                move_vec.z -= 1.0;
            }
            if map.backward.is_being_pressed(key) {
                move_vec.z += 1.0;
            }
            if map.left.is_being_pressed(key) {
                move_vec.x -= 1.0;
            }
            if map.right.is_being_pressed(key) {
                move_vec.x += 1.0;
            }

            if map.up.is_being_pressed(key) {
                move_vec.y += 1.0;
            }
            if map.down.is_being_pressed(key) {
                move_vec.y -= 1.0;
            }

            boost = if map.boost.is_being_pressed(key) {
                1.
            } else {
                0.
            };
        }

        for event in mouse_motion_events.iter() {
            delta += event.delta;
        }

        let move_vec = self.rig.final_transform.rotation.zero_y_rotation()
            * move_vec.clamp_length_max(1.0)
            * boost_mult.powf(boost);

        let window = windows.get_primary().unwrap();
        if window.cursor_locked() {
            self.rig.driver_mut::<YawPitch>().rotate_yaw_pitch(
                -0.1 * delta.x * sensitivity.x,
                -0.1 * delta.y * sensitivity.y,
            );

            self.rig
                .driver_mut::<Position>()
                .translate(move_vec * time_delta_seconds * 10.0);
        }
    }
}

pub struct FpsSettings {
    pub transform: dolly::transform::Transform,
}

impl WithRigSettings<FpsSettings> for Fps {
    fn init(settings: FpsSettings) -> Self {
        let mut yp = YawPitch::new();
        yp.set_rotation_quat(settings.transform.rotation);
        Fps {
            rig: CameraRig::builder()
                .with(Position {
                    position: settings.transform.position,
                })
                .with(Rotation {
                    rotation: settings.transform.rotation,
                })
                .with(yp)
                .with(Smooth::new_position_rotation(1.0, 0.5))
                .build(),
        }
    }
}

impl RigDriver for Fps {
    fn update(&mut self, params: RigUpdateParams) -> dolly::transform::Transform {
        let t = self.rig.update(params.delta_time_seconds);
        dolly::transform::Transform {
            position: t.position,
            rotation: t.rotation,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
