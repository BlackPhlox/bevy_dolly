use bevy::prelude::*;
use bevy_dolly::*;


#[derive(Component)]
struct MainCamera;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum Camera {
    FollowPlayer,
    FollowSheep,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyPlugin)
        .add_startup_system(setup)
        .add_system(rotator_system)
        .add_state(Camera::FollowSheep)
        .add_system(switch_camera_rig_system)
        .add_system_set(
            SystemSet::on_update(Camera::FollowPlayer).with_system(follow_player_system),
        )
        .add_system_set(SystemSet::on_update(Camera::FollowSheep).with_system(follow_sheep_system))
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
            cell.spawn_scene(asset_server.load("poly_dolly.gltf#Scene0"));
        })
        .insert(Rotates);

    commands.spawn().insert(
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

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 1., 5.0)
                .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y),
            ..Default::default()
        })
        .insert(MainCamera);

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

#[allow(clippy::type_complexity)]
fn follow_player_system(
    time: Res<Time>,
    mut query: QuerySet<(
        QueryState<&mut Transform, With<MainCamera>>,
        QueryState<&mut Transform, With<CtrlMove>>,
        QueryState<&mut CameraRig>,
    )>,
) {
    let mut q1 = query.q1();
    let player = q1.single_mut();
    

    let mut q2 = query.q2();
    let mut rig = q2.single_mut();

    rig.driver_mut::<Position>().position = player.translation;
    rig.driver_mut::<Rotation>().rotation = player.rotation;
    rig.driver_mut::<LookAt>().target = player.translation + Vec3::Y + Vec3::new(0., -1., 0.);

    let transform = rig.update(time.delta_seconds());

    let cam = query.q0().single_mut();
    *cam = transform;
}

#[allow(clippy::type_complexity)]
fn follow_sheep_system(
    time: Res<Time>,
    mut query: QuerySet<(
        QueryState<&mut Transform, With<MainCamera>>,
        QueryState<&mut Transform, With<Rotates>>,
        QueryState<&mut CameraRig>,
    )>,
) {
    
    
    let mut player = query.q0().single_mut();

    let mut q2 = query.q2();
    let mut rig = q2.single_mut();

    rig.driver_mut::<Position>().position = player.translation;
    rig.driver_mut::<Rotation>().rotation = player.rotation;
    rig.driver_mut::<LookAt>().target = player.translation + Vec3::Y;

    let transform = rig.update(time.delta_seconds());

    let mut cam = query.q0().single_mut();
    *cam = transform;
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

#[allow(unused_must_use)]
fn switch_camera_rig_system(
    mut camera: ResMut<State<Camera>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        let result = if camera.current().eq(&Camera::FollowPlayer) {
            Camera::FollowSheep
        } else {
            Camera::FollowPlayer
        };

        println!("{:?}", result);
        camera.set(result);
    }
}
