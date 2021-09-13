use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_dolly::drivers::fps::{Fps, Vec3KeyMapWithBoost};
use bevy_dolly::{cam_ctrl::DollyCursorGrab, Transform2Bevy, Transform2Dolly};

use dolly::glam::Vec3;
use dolly::prelude::{CameraRig, Smooth};

struct MainCamera;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyCursorGrab)
        .add_startup_system(setup.system())
        .add_system(update_camera.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    commands
        .spawn_bundle((
            Transform {
                translation: bevy::math::Vec3::new(0., 0.2, 0.),
                ..Default::default()
            },
            GlobalTransform::identity(),
        ))
        .with_children(|cell| {
            cell.spawn_scene(asset_server.load("poly_dolly.gltf#Scene0"));
        })
        .id();

    let translation = [-2.0f32, 2.0f32, 5.0f32];
    let transform =
        Transform::from_translation(bevy::math::Vec3::from_slice_unaligned(&translation))
            .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y);

    let rotation = transform.transform_2_dolly().rotation;

    commands.spawn().insert(
        CameraRig::builder()
            .with(Fps::init(dolly::transform::Transform {
                position: Vec3::from_slice(&translation),
                rotation,
            }))
            .with(Smooth::new_position_rotation(1.0, 0.1))
            .build(),
    );

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform,
            ..Default::default()
        })
        .insert(MainCamera);

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn update_camera(
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
        Vec3KeyMapWithBoost::default(),
    );

    let transform = rig.update(time_delta_seconds);
    let (mut cam, _) = query.q0_mut().single_mut().unwrap();

    cam.transform_2_bevy(transform);
}
