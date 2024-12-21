use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_dolly::{drivers::follow::MovableLookAt, prelude::*};

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                Dolly::<MainCamera>::update_active,
                update_camera,
                rotator_system,
            ),
        )
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
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5., 5.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3)))
    ));

    let start_pos = Vec3::new(0., 0., 0.);

    let poly_dolly = asset_server.load(GltfAssetLabel::Scene(0).from_asset("poly_dolly.gltf"));

    commands.spawn((
        SceneRoot(poly_dolly),
        Transform::from_xyz(0., 0.2, 0.),
        Rotates
    ));

    commands.spawn((
        MainCamera,
        Rig::builder()
            .with(MovableLookAt::from_position_target(start_pos))
            .build(),
        Camera3d::default(),
        Transform::from_xyz(-2.0, 1., 5.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));

    // light
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0)
    ));
}

fn update_camera(q0: Query<&Transform, With<Rotates>>, mut q1: Query<&mut Rig>) {
    let player = q0.single().to_owned();
    let mut rig = q1.single_mut();

    rig.driver_mut::<MovableLookAt>()
        .set_position_target(player.translation, player.rotation);
}

#[derive(Component)]
struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * PI / 20.0) * time.delta_secs(),
        )) * *transform;
    }
}
