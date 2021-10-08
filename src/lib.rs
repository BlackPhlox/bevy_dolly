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
        app.init_resource::<DollyControlConfig>()
            .add_system_to_stage(CoreStage::PreUpdate, init_camera_system)
            .add_system_to_stage(CoreStage::PreUpdate, update_look_at_rigs_system)
            .add_system_to_stage(CoreStage::PreUpdate, update_control_system)
            // This will update all camera rig position and rotation
            .add_system_to_stage(CoreStage::Update, apply_rigs_system);
    }
}

/// Configuration Resource for Dolly Controlled Rigs
pub struct DollyControlConfig {
    pub speed: f32,
    pub key_rotation: f32,
    pub boost_multiplyer: f32,
    pub sensitivity: Vec2,
}

impl Default for DollyControlConfig {
    fn default() -> Self {
        Self {
            speed: 10.0,
            key_rotation: 15.0,
            boost_multiplyer: 5.0,
            sensitivity: Vec2::splat(1.0),
        }
    }
}

/// Listen for new Rigs
/// Add position and rotation info if needed
fn init_camera_system(mut query: Query<(&mut Transform, &mut Rig), Added<Rig>>) {
    for (mut transform, mut rig) in query.iter_mut() {
        // Update Position if it was set to default with the transform
        if let Some(d) = rig.get_driver_mut::<Position>() {
            if d.init_set {
                d.position = transform.translation;
            }
        }

        // Update Rotation if it was set to default with the transform
        if let Some(d) = rig.get_driver_mut::<Rotation>() {
            if d.init_set {
                d.rotation = transform.rotation;
            }
        }

        // for d in rig.drivers.iter() {
        //     info!("driver: {:?}", d);
        // }

        // Update once with no time to setup if needed
        *transform = rig.update(0.0);
    }
}

fn update_look_at_rigs_system(mut rig_query: Query<&mut Rig>, transform_query: Query<&Transform>) {
    for mut rig in rig_query.iter_mut() {
        // Update LookAt Drivers
        if let Some(d) = rig.get_driver_mut::<LookAt>() {
            if let Some(target_entity) = d.target_entity {
                if let Ok(target_transfrom) = transform_query.get(target_entity) {
                    d.target_transform = Some(*target_transfrom);
                }
            }
        }

        // Update Follow Drivers
        if let Some(d) = rig.get_driver_mut::<Follow>() {
            match transform_query.get(d.target_entity) {
                Ok(t) => d.target = t.clone(),
                Err(_) => (),
            }
        }
    }
}

fn apply_rigs_system(time: Res<Time>, mut query: Query<(&mut Transform, &mut Rig)>) {
    for (mut transform, mut rig) in query.iter_mut() {
        *transform = rig.update(time.delta_seconds());
    }
}


// Handle user input
// NOTE: This is only run for DollyControlCameraBundles, not DollyCameraBundles due
// to CameraActions component
fn update_control_system(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    config: Res<DollyControlConfig>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Rig, &CameraActions)>,
) {
    for (mut rig, camera_keys) in query.iter_mut() {
        // Update position
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

        // Move relative to the direction of the camera
        move_vec = rig.final_transform.rotation * move_vec.clamp_length_max(1.0) * boost;

        if let Some(d) = rig.get_driver_mut::<Position>() {
            d.position += move_vec * time.delta_seconds() * config.speed;
        }

        // Update rotation
        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            delta += event.delta;
        }
        if camera_keys.pressed(CameraAction::RotateLeft, &keys) {
            delta.x -= 10.0;
        }
        if camera_keys.pressed(CameraAction::RotateRight, &keys) {
            delta.x += 10.0;
        }

        if let Some(d) = rig.get_driver_mut::<YawPitch>() {
            d.rotate_yaw_pitch(
                -0.1 * delta.x * config.sensitivity.x,
                -0.1 * delta.y * config.sensitivity.y,
            );
        }
    }
}
