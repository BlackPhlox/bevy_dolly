use bevy::ecs::schedule::ShouldRun;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use dolly::glam::Vec3;
use dolly::prelude::CameraRig;

use crate::cam_ctrl::DollyCursorGrab;
use crate::drivers::fps::{Fps, FpsSettings, Vec3KeyMapWithBoost};
use crate::{CustomBuild, Transform2Bevy, Transform2Dolly};

pub struct DollyFps;
impl Plugin for DollyFps {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<DollyFpsConfig>()
            .add_plugin(DollyCursorGrab)
            .add_startup_system(setup.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(use_fps.system())
                    .with_system(update_camera.system()),
            );
    }
}

fn use_fps(config: Res<DollyFpsConfig>) -> ShouldRun {
    if config.enabled {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub struct DollyFpsConfig {
    pub enabled: bool,
    pub map: Vec3KeyMapWithBoost,
}

impl Default for DollyFpsConfig {
    fn default() -> Self {
        DollyFpsConfig {
            enabled: true,
            map: Vec3KeyMapWithBoost::default(),
        }
    }
}

struct MainCamera;

fn setup(mut commands: Commands) {
    let translation = [-2.0f32, 2.0f32, 5.0f32];
    let transform =
        Transform::from_translation(bevy::math::Vec3::from_slice_unaligned(&translation))
            .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y);

    let rotation = transform.transform_2_dolly().rotation;

    commands.spawn().insert(
        CameraRig::builder()
            .with_rig::<Fps, FpsSettings>(FpsSettings {
                transform: dolly::transform::Transform {
                    position: Vec3::from_slice(&translation),
                    rotation,
                },
            })
            .build(),
    );

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform,
            ..Default::default()
        })
        .insert(MainCamera);
}

#[allow(clippy::type_complexity)]
fn update_camera(
    config: Res<DollyFpsConfig>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mouse_motion_events: EventReader<MouseMotion>,
    mut query: QuerySet<(
        Query<(&mut Transform, With<MainCamera>)>,
        Query<&mut CameraRig>,
    )>,
) {
    let time_delta_seconds: f32 = time.delta_seconds();
    let sensitivity = Vec2::splat(1.0);

    let mut rig = query.q1_mut().single_mut().unwrap();
    rig.driver_mut::<Fps>().update(
        time,
        keys,
        windows,
        mouse_motion_events,
        sensitivity,
        &config.map,
    );

    let transform = rig.update(time_delta_seconds);
    let (mut cam, _) = query.q0_mut().single_mut().unwrap();

    cam.transform_2_bevy(transform);
}
