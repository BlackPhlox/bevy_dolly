pub mod bundle;
pub mod drivers;
pub mod rig;

use bevy::{input::mouse::MouseMotion, prelude::*};
use bundle::*;
use drivers::*;
use rig::*;

pub mod prelude {
    pub use crate::{bundle::*, drivers::*, rig::*, *};
}

pub struct DollyPlugin;
impl Plugin for DollyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DollyConfig>()
            // Adds position and rotation drivers
            .add_system(init_camera_system)
            // This will update all camera rig position and rotation
            .add_system(update_camera_rigs_system)
            // Handles input for control bundles
            .add_system(update_control_system);
    }
}

pub struct DollyConfig {
    pub speed: f32,
    pub target: Option<Entity>,
}

impl Default for DollyConfig {
    fn default() -> Self {
        Self {
            speed: 10.0,
            target: None,
        }
    }
}

/// Add position and rotation from init transform
fn init_camera_system(
    mut query: Query<(&Transform, &mut CameraRig, &mut RigBuilder), Added<CameraRig>>,
) {
    for (transform, mut rig, mut builder) in query.iter_mut() {
        rig.drivers.insert(
            0,
            Box::new(Position {
                position: transform.translation.clone(),
            }),
        );
        rig.drivers.insert(
            1,
            Box::new(Rotation {
                rotation: transform.rotation.clone(),
            }),
        );

        rig.update(0.0);
    }
}

// TODO: Could filter by Bundle, but this may make it more useable later
fn update_camera_rigs_system(time: Res<Time>, mut query: Query<(&mut Transform, &mut CameraRig)>) {
    for (mut t, mut rig) in query.iter_mut() {
        *t = rig.update(time.delta_seconds());
    }
}

fn update_control_system(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    config: Res<DollyConfig>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraRig, &CameraActions)>,
) {
    for (mut transform, mut rig, camera_keys) in query.iter_mut() {
        let boost_mult: f32 = 5.0;
        let sensitivity = Vec2::splat(1.0);

        let mut move_vec = Vec3::ZERO;

        if camera_keys.pressed(CameraAction::Forward, &keys) {
            move_vec.z -= 1.0;
        }
        if camera_keys.pressed(CameraAction::Backward, &keys) {
            move_vec.z += 1.0;
        }
        if camera_keys.pressed(CameraAction::Left, &keys) {
            move_vec.x -= 1.0;
        }
        if camera_keys.pressed(CameraAction::Right, &keys) {
            move_vec.x += 1.0;
        }
        if camera_keys.pressed(CameraAction::Up, &keys) {
            move_vec.y += 1.0;
        }
        if camera_keys.pressed(CameraAction::Down, &keys) {
            move_vec.y -= 1.0;
        }

        let boost = match camera_keys.pressed(CameraAction::Boost, &keys) {
            true => 1.0,
            false => 0.0,
        };

        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            delta += event.delta;
        }
        let move_vec =
            rig.final_transform.rotation * move_vec.clamp_length_max(1.0) * boost_mult.powf(boost);

        if let Some(d) = rig.get_driver_mut::<Position>() {
            d.position += move_vec * time.delta_seconds() * config.speed;
        }

        if let Some(d) = rig.get_driver_mut::<YawPitch>() {
            d.rotate_yaw_pitch(
                -0.1 * delta.x * sensitivity.x,
                -0.1 * delta.y * sensitivity.y,
            );
        }

        //transform.translation = move_vec * time.delta_seconds() * config.speed;
    }
}
