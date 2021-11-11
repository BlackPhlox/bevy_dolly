use std::f32::consts::{FRAC_2_PI, FRAC_PI_2, FRAC_PI_4, FRAC_PI_8, PI};

use bevy::prelude::*;
use bevy_dolly::{DollyPlugins, Transform2Bevy, Transform2Dolly, UpdateYawPitch, ZeroedYRotation};
use dolly::prelude::{Arm, CameraRig, LockRotation, LookAt, Rotation, Smooth, YawPitch};

struct MainCamera;

struct Turret;
struct TurretBarrel;
struct TurretRig;
struct TurretBarrelRig;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(update_turret.system())
        .add_system(update_barrels.system())
        .add_system(rotator_system.system())
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
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 3.5, 8.0)
                .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y),
            ..Default::default()
        })
        .insert(MainCamera);

    let poly_dolly_transform = Transform {
        translation: Vec3::new(4., 2., 0.),
        rotation: bevy::math::Quat::from_rotation_y(PI),
        ..Default::default()
    };

    commands
        .spawn_bundle((poly_dolly_transform, GlobalTransform::identity()))
        .with_children(|cell| {
            cell.spawn_scene(asset_server.load("poly_dolly.gltf#Scene0"));
        })
        .insert(Rotates);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(1., 0.5, 1.))),
        material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        transform: Transform::from_xyz(0.0, 0.25, 0.0),
        ..Default::default()
    });

    commands
        .spawn()
        .insert(
            CameraRig::builder()
                .with(Rotation::new(dolly::glam::Quat::IDENTITY))
                .with(Smooth::new_rotation(1.5))
                .with(Arm::new(dolly::glam::Vec3::new(0., 0.5, 0.)))
                .with(
                    LookAt::new(
                        poly_dolly_transform.transform_2_dolly().position + dolly::glam::Vec3::Y,
                    )
                    .tracking_smoothness(1.25)
                    .tracking_predictive(true),
                )
                .with(LockRotation::new().y())
                .build(),
        )
        .insert(TurretRig);

    commands
        .spawn()
        .insert(
            CameraRig::builder()
                .with(Rotation::new(dolly::glam::Quat::IDENTITY))
                .with(Smooth::new_rotation(1.5))
                //.with(Arm::new(dolly::glam::Vec3::new(0., 0.5, 0.)))
                .build(),
        )
        .insert(TurretBarrelRig);

    commands
        .spawn_bundle((
            Transform {
                translation: bevy::math::Vec3::new(0., 0.5, 0.),
                ..Default::default()
            },
            GlobalTransform::identity(),
        ))
        .with_children(|cell| {
            cell.spawn_scene(asset_server.load("turret_base.gltf#Scene0"));
        })
        .insert(Turret)
        .with_children(|parent| {
            parent
                .spawn_bundle((
                    Transform {
                        ..Default::default()
                    },
                    GlobalTransform::identity(),
                ))
                .insert(TurretBarrel)
                .with_children(|cell| {
                    cell.spawn_scene(asset_server.load("turret_barrels.gltf#Scene0"));
                });
        });

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn update_turret(
    time: Res<Time>,
    mut query: QuerySet<(
        Query<(&mut Transform, With<Turret>)>,
        Query<(&mut CameraRig, With<TurretRig>)>,
        Query<(&mut Transform, With<Rotates>)>,
    )>,
) {
    let poly_dolly = query.q2_mut().single_mut().unwrap();
    let target = poly_dolly.0.transform_2_dolly().position + dolly::glam::Vec3::Y;

    let mut rig = query.q1_mut().single_mut().unwrap();
    let camera_driver = rig.0.driver_mut::<LookAt>();
    camera_driver.target = target;

    let transform = rig.0.update(time.delta_seconds());

    let (mut turret, _) = query.q0_mut().single_mut().unwrap();
    turret.transform_2_bevy(transform);
}

fn update_barrels(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: QuerySet<(
        Query<(&mut Transform, With<TurretBarrel>)>,
        Query<(&mut CameraRig, With<TurretBarrelRig>)>,
    )>,
) {
    let mut rig = query.q1_mut().single_mut().unwrap();
    let mut camera_driver = rig.0.driver_mut::<Rotation>();

    if keys.just_pressed(KeyCode::W) {
        camera_driver.rotation =
            camera_driver.rotation + dolly::glam::Quat::from_rotation_x(FRAC_PI_8 / 1.5);
        camera_driver.rotation = camera_driver.rotation.normalize();
    }
    if keys.just_pressed(KeyCode::S) {
        camera_driver.rotation =
            camera_driver.rotation + dolly::glam::Quat::from_rotation_x(-FRAC_PI_8 / 1.5);
        camera_driver.rotation = camera_driver.rotation.normalize();
    }

    let transform = rig.0.update(time.delta_seconds());
    let (mut cam, _) = query.q0_mut().single_mut().unwrap();

    cam.transform_2_bevy(transform);
}

struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(bevy::math::Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}
