pub mod bundle;
pub mod camera_rig;
pub mod drivers;

use bevy::{input::mouse::MouseMotion, prelude::*};
use camera_rig::*;
use drivers::*;
use bundle::*;

pub mod prelude {
    pub use crate::{bundle::*, camera_rig::*, drivers::*, *};
}

pub struct DollyPlugin;
impl Plugin for DollyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DollyConfig>();
            //.add_system(init_listen_system);
            //.add_system(update_camera_system);
    }
}

pub struct DollyConfig {
    pub speed: f32,
    pub target: Option<Entity>,
}

impl Default for DollyConfig {
    fn default() -> Self {
        Self {
            speed: 4.,
            target: None,
        }
    }
}

// TODO: Could filter by Bundle, but this may make it more usable later
fn init_listen_system(
    mut query: Query<(&mut Transform, &mut CameraRig), Added<CameraRig>>, )
{
    for (t, mut rig) in query.iter_mut() {

        // TODO: these will be going away, just keeping things going as I refactor
        let mut yaw_pitch = YawPitch::new();
        yaw_pitch.set_rotation_quat(t.rotation);

        rig.drivers.push(Box::new(Position {
            position: t.translation,
        }));
        rig.drivers.push(Box::new(Rotation {
            rotation: t.rotation,
        }));
        rig.drivers.push(Box::new(yaw_pitch));

        info!("rig created");
    }
}


fn update_camera_system(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    //config: Res<DollyConfig>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraRig, &CameraActionMap )>,
) {
    for (mut t, mut rig, camera_keys) in query.iter_mut() {
        let time_delta_seconds: f32 = time.delta_seconds();
        let boost_mult: f32 = 5.0;
        let sensitivity = Vec2::splat(1.0);

        let mut move_vec = Vec3::ZERO;

        if camera_keys.pressed( CameraAction::Forward, &keys ) {
            move_vec.z += 1.0;
        }
        if camera_keys.pressed( CameraAction::Backward, &keys ) {
            move_vec.z -= 1.0;
        }
        if camera_keys.pressed( CameraAction::Left, &keys ) {
            move_vec.x -= 1.0;
        }
        if camera_keys.pressed( CameraAction::Right, &keys ) {
            move_vec.x += 1.0;
        }
        if camera_keys.pressed( CameraAction::Up, &keys ) {
            move_vec.y += 1.0;
        }
        if camera_keys.pressed( CameraAction::Down, &keys ) {
            move_vec.y -= 1.0;
        }

        let boost = match camera_keys.pressed( CameraAction::Boost, &keys ) {
            true => 1.0,
            false => 0.0,
        };

        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            delta += event.delta;
        }

        let move_vec =
            rig.final_transform.rotation * move_vec.clamp_length_max(1.0) * boost_mult.powf(boost);

        let window = windows.get_primary().unwrap();
        if window.cursor_locked() {
            rig.driver_mut::<YawPitch>().rotate_yaw_pitch(
                -0.1 * delta.x * sensitivity.x,
                -0.1 * delta.y * sensitivity.y,
            );
            rig.driver_mut::<Position>()
                .translate(move_vec * time_delta_seconds * 10.0);
        }

        *t = rig.update(time_delta_seconds);
        info!("update rig");
    }
}
