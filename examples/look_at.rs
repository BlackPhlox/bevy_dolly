//Currently not working
use bevy::prelude::*;
use bevy_dolly::{Transform2Bevy, Transform2Dolly};
use dolly::glam::Vec3;
use dolly::prelude::{CameraRig, LookAt, Positional};

struct MainCamera;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(rotator_system.system())
        .add_system(update_camera.system())
        .run();
}

struct Player;

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

    let start_pos = Vec3::new(0., 0., 2.);

    commands
        .spawn_bundle((
            Transform {
                translation: bevy::math::Vec3::new(0., 0.2, 0.),
                ..Default::default()
            },
            GlobalTransform::identity(),
        ))
        .with_children(|cell| {
            cell.spawn_bundle((
                Transform {
                    translation: bevy::math::Vec3::new(0., 0.0, 2.),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            ))
            .with_children(|cell2| {
                cell2.spawn_scene(asset_server.load("sheep.gltf#Scene0"));
            })
            .insert(Player);
        })
        .insert(Rotates);

    commands.spawn().insert(
        CameraRig::builder()
            .with(Positional::new(Vec3::Y * 3.0))
            .with(LookAt::new(start_pos))
            .build(),
    );

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 1., 5.0)
                .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y),
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
    mut query: QuerySet<(
        Query<(&mut Transform, With<MainCamera>)>,
        Query<(&mut Transform, With<Player>)>,
        Query<&mut CameraRig>,
    )>,
) {
    let (player, _) = query.q1_mut().single_mut().unwrap();
    query
        .q2_mut()
        .single_mut()
        .unwrap()
        .driver_mut::<LookAt>()
        .target = player.transform2dolly().translation;

    let transform = query
        .q2_mut()
        .single_mut()
        .unwrap()
        .update(time.delta_seconds());
    let (mut cam, _) = query.q0_mut().single_mut().unwrap();

    cam.transform2bevy(transform);
}

struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(bevy::math::Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}
