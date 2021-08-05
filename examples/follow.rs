use bevy::prelude::*;
use dolly::glam::{Quat, Vec3};
use dolly::prelude::{Arm, CameraRig, LookAt, Positional, Smooth};

struct Dolly {
    rigs: CameraRig,
}

struct MainCamera;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(rotator_system.system().label("rotate"))
        .add_system(update_camera.system().after("rotate"))
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

    let start_pos = Vec3::new(0., 0., 0.);

    commands
        .spawn_bundle((
            Transform {
                translation: bevy::math::Vec3::new(0., 0.2, 0.),
                ..Default::default()
            },
            GlobalTransform::identity(),
        ))
        .with_children(|cell| {
            cell.spawn_scene(asset_server.load("sheep.gltf#Scene0"));
        })
        .insert(Rotates);

    let camera = CameraRig::builder()
        .with(Positional::new(start_pos))
        .with(Smooth::new_move(1.25).predictive(1.0))
        .with(Arm::new(Vec3::new(0.0, 1.5, -3.5)))
        .with(Smooth::new_move(2.5))
        .with(
            LookAt::new(start_pos + Vec3::Y)
                .smoothness(1.25)
                .predictive(1.0),
        )
        .build();

    commands.insert_resource(Dolly { rigs: camera });

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
//QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
fn update_camera(
    mut dolly: ResMut<Dolly>,
    mut query: QuerySet<(
        Query<(&mut Transform, With<MainCamera>)>,
        Query<(&mut Transform, With<Rotates>)>,
    )>,
) {
    let (player, _) = query.q1_mut().single_mut().unwrap();
    let time_delta_seconds : f32 = 0.1;

    let player_translation = Vec3::new(
        player.translation.x,
        player.translation.y,
        player.translation.z,
    );
    let player_rotation = Quat::from_xyzw(
        player.rotation.x,
        player.rotation.y,
        player.rotation.z,
        player.rotation.w,
    );

    dolly
        .rigs
        .driver_mut::<Positional>()
        .set_position_rotation(player_translation, player_rotation);
    dolly.rigs.driver_mut::<LookAt>().target = player_translation + Vec3::Y;

    let transform = dolly.rigs.update(time_delta_seconds);
    let translation = transform.translation;
    let rotation = transform.rotation;

    let (mut cam, _) = query.q1_mut().single_mut().unwrap();
    cam.translation = bevy::math::Vec3::new(translation.x, translation.y, translation.z);
    cam.rotation = bevy::math::Quat::from_xyzw(rotation.x, rotation.y, rotation.z, rotation.w);
}

struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(bevy::math::Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}
