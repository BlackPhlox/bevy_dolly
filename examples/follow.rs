use bevy::prelude::*;
use bevy_dolly::*;
#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotator_system)
        .add_system(update_camera_system)
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
                translation: Vec3::new(0., 0.2, 0.),
                ..Default::default()
            },
            GlobalTransform::identity(),
        ))
        .with_children(|cell| {
            cell.spawn_scene(asset_server.load("poly_dolly.gltf#Scene0"));
        })
        .insert(Rotates);

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 1., 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(
                // TODO: can we have two smooths?
            CameraRig::builder()
                .with(Position::new(start_pos))
                .with(Rotation::new(Quat::IDENTITY))
                .with(Smooth::new_position(1.25).predictive(true))
                .with(Arm::new(Vec3::new(0.0, 1.5, -3.5)))
                .with(Smooth::new_position(2.5))
                .with(
                    LookAt::new(start_pos + Vec3::Y)
                        .tracking_smoothness(1.25)
                        .tracking_predictive(true),
                )
                .build(),
        );

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        mesh: meshes.add(Mesh::from(shape::Capsule {
            ..Default::default()
        })),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

#[allow(clippy::type_complexity)]
fn update_camera_system(
    time: Res<Time>,
    mut query: QuerySet<(
        QueryState<&Transform, With<Rotates>>,
        QueryState<(&mut Transform, &mut CameraRig)>,
    )>,
) {
    // TODO: Find cleaner way
    let mut target_pos = Vec3::default();
    let mut target_rotation = Quat::default();
    let mut found = false;
    for target in query.q0().iter() {
        target_pos = target.translation;
        target_rotation = target.rotation;
        found = true;
    }

    if found {
        for (mut cam, mut rig) in query.q1().iter_mut() {
            rig.driver_mut::<Position>().position = target_pos;
            rig.driver_mut::<Rotation>().rotation = target_rotation;
            rig.driver_mut::<LookAt>().target = target_pos + Vec3::Y;
            *cam = rig.update(time.delta_seconds());
        }
    }
}

#[derive(Component)]
struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}
