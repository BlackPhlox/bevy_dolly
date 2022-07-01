use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_dolly::{drivers::follow::MovableLookAt, prelude::*};

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotator_system)
        .add_system(update_camera)
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
        ..default()
    });

    let start_pos = dolly::glam::Vec3::new(0., 0., 0.);

    commands
        .spawn_bundle((
            Transform {
                translation: Vec3::new(0., 0.2, 0.),
                ..default()
            },
            GlobalTransform::identity(),
        ))
        .with_children(|cell| {
            cell.spawn_bundle(SceneBundle {
                scene: asset_server.load("poly_dolly.gltf#Scene0"),
                ..default()
            });
        })
        .insert(Rotates);

    commands.spawn().insert(
        Rig::builder()
            .with(MovableLookAt::from_position_target(start_pos))
            .build(),
    );

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 1., 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(MainCamera);

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn update_camera(
    time: Res<Time>,
    mut query: ParamSet<(
        Query<(&mut Transform, With<MainCamera>)>,
        Query<(&Transform, With<Rotates>)>,
        Query<&mut Rig>,
    )>,
) {
    let p1 = query.p1();
    let player = p1.single().0.to_owned();

    let mut p2 = query.p2();
    let mut rig = p2.single_mut();

    rig.driver_mut::<MovableLookAt>()
        .set_position_target(player.translation, player.rotation);

    let transform = rig.update(time.delta_seconds());

    query.p0().single_mut().0.transform_2_bevy(transform);
}

#[derive(Component)]
struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}
