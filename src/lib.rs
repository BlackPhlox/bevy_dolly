#![feature(derive_default_enum)]
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

            .add_system(init_camera_system)
            // This will update all camera rig position and rotation
            .add_system(update_camera_rigs_system)
            // Handles input for control bundles
            .add_system(update_control_system);
    }
}

pub struct DollyConfig {
    pub speed: f32,
    pub boost_multiplyer: f32,
    pub sensitivity: Vec2,
    pub target: Option<Entity>,
}

impl Default for DollyConfig {
    fn default() -> Self {
        Self {
            speed: 10.0,
            target: None,
            boost_multiplyer: 5.0,
            sensitivity: Vec2::splat(1.0),
        }
    }
}

/// Add position and rotation from init transform
fn init_camera_system(
    mut query: Query<(&Transform, &mut Rig, &mut RigBuilder), Added<Rig>>,
) {
    for (transform, mut rig, mut builder) in query.iter_mut() {

        // Build our rig
        rig.drivers.append(&mut builder.drivers);

        if let Some(d) = rig.get_driver_mut::<Position>() {
            if d.transform_set {
                info!("set trans {:?}", transform.translation);
                d.position = transform.translation;
            }
        }

        if let Some(d) = rig.get_driver_mut::<Rotation>() {
            if d.transform_set {
                info!("set rot {:?}", transform.rotation);
                d.rotation = transform.rotation;
            }
        }

        rig.update(0.0);
    }
}

// TODO: Could filter by Bundle, but this may make it more useable later
fn update_camera_rigs_system(time: Res<Time>, mut query: Query<(&mut Transform, &mut Rig)>) {
    for (mut t, mut rig) in query.iter_mut() {
        *t = rig.update(time.delta_seconds());
    }
}

fn update_control_system(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    config: Res<DollyConfig>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Rig, &CameraActions)>,
) {
    for (mut rig, camera_keys) in query.iter_mut() {


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
            true => config.boost_multiplyer,
            false => 1.0,
        };

        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            delta += event.delta;
        }
        move_vec = rig.final_transform.rotation * move_vec.clamp_length_max(1.0) * boost;

        if let Some(d) = rig.get_driver_mut::<Position>() {
            d.position += move_vec * time.delta_seconds() * config.speed;
        }
        if let Some(d) = rig.get_driver_mut::<YawPitch>() {
            d.rotate_yaw_pitch(
                -0.1 * delta.x * config.sensitivity.x,
                -0.1 * delta.y * config.sensitivity.y,
            );
        }
    }
}
