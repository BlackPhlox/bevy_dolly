use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_dolly::{drivers::follow::MovableLookAt, prelude::*};

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotator_system)
        .add_system(update_camera)
        .add_system(Dolly::<MainCamera>::update_active)
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
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            ..Default::default()
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    let start_pos = Vec3::new(0., 0., 0.);

    commands.spawn((
        Rotates,
        SceneBundle {
            scene: asset_server.load("poly_dolly.gltf#Scene0"),
            transform: Transform {
                translation: Vec3::new(0., 0.2, 0.),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        MainCamera,
        Rig::builder()
            .with(MovableLookAt::from_position_target(start_pos))
            .build(),
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 1., 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn update_camera(q0: Query<(&Transform, With<Rotates>)>, mut q1: Query<&mut Rig>) {
    let player = q0.single().0.to_owned();
    let mut rig = q1.single_mut();

    rig.driver_mut::<MovableLookAt>()
        .set_position_target(player.translation, player.rotation);
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
