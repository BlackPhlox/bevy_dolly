use std::f32::consts::FRAC_PI_8;

use bevy::prelude::*;
use bevy_dolly::{DollyPlugins, Transform2Bevy, Transform2Dolly, UpdateYawPitch, ZeroedYRotation};
use dolly::prelude::{Arm, CameraRig, Rotation, Smooth, YawPitch};

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
            transform: Transform::from_xyz(0.0, 2.0, 5.0)
                .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y),
            ..Default::default()
        })
        .insert(MainCamera);

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
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: QuerySet<(
        Query<(&mut Transform, With<Turret>)>,
        Query<(&mut CameraRig, With<TurretRig>)>,
    )>,
) {
    let mut rig = query.q1_mut().single_mut().unwrap();
    let camera_driver = rig.0.driver_mut::<Rotation>();

    let (_, mut rotation) = camera_driver.rotation.to_axis_angle();
    if keys.pressed(KeyCode::D) {
        if rotation > std::f32::consts::FRAC_PI_2 * 4.0 - 0.05 {
            rotation = 0.0;
        }
        rotation += 0.05
    }
    if keys.pressed(KeyCode::A) {
        if rotation < 0.05 {
            rotation = std::f32::consts::FRAC_PI_2 * 4.0;
        }
        rotation -= 0.05
    }

    camera_driver.rotation = dolly::glam::Quat::from_rotation_y(rotation);

    let transform = rig.0.update(time.delta_seconds());
    let (mut cam, _) = query.q0_mut().single_mut().unwrap();

    cam.transform_2_bevy(transform);
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
            camera_driver.rotation + dolly::glam::Quat::from_rotation_z(-FRAC_PI_8 / 1.5);
        camera_driver.rotation = camera_driver.rotation.normalize();
    }
    if keys.just_pressed(KeyCode::S) {
        camera_driver.rotation =
            camera_driver.rotation + dolly::glam::Quat::from_rotation_z(FRAC_PI_8 / 1.5);
        camera_driver.rotation = camera_driver.rotation.normalize();
    }

    let transform = rig.0.update(time.delta_seconds());
    let (mut cam, _) = query.q0_mut().single_mut().unwrap();

    cam.transform_2_bevy(transform);
}
