#![allow(clippy::type_complexity)]
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_dolly::prelude::*;

#[derive(Component)]
struct MainCamera;

// In this example we are going to switch our look at target
// All you need to do is set a LookAt driver target_entity
// and its will track it

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum Camera {
    #[default]
    FollowPlayer,
    FollowSheep,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DollyPosCtrl))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                Dolly::<MainCamera>::update_active,
                rotator_system,
                switch_camera_rig,
                follow_player.run_if(in_state(Camera::FollowPlayer)),
                follow_sheep.run_if(in_state(Camera::FollowSheep)),
            ),
        )
        .init_state::<Camera>()
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
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    let start_pos = Vec3::new(0., 0., 0.);

    let poly_dolly = asset_server.load(GltfAssetLabel::Scene(0).from_asset("poly_dolly.gltf"));

    commands.spawn((
        Rotates,
        SceneRoot(poly_dolly),
        Transform {
            translation: Vec3::new(0., 0.2, 0.),
            ..default()
        },
    ));

    commands.spawn((
        MainCamera,
        Rig::builder()
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
    ));

    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(-2.0, 1., 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // light
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));

    //info!(" Use 1, 2, 3, 4 to target different sheep");
    //info!(" Use Q and E to turn the sheep");
    info!("Press C to toggle between the default player and the sheep");
}

fn follow_player(query: Query<&Transform, With<DollyPosCtrlMove>>, mut q: Query<&mut Rig>) {
    if let Ok(p) = query.get_single() {
        let mut rig = q.single_mut();
        rig.driver_mut::<Position>().position = p.translation;
        rig.driver_mut::<Rotation>().rotation = p.rotation;
        rig.driver_mut::<LookAt>().target = p.translation + Vec3::Y + Vec3::new(0., -1., 0.);
    }
}

fn follow_sheep(query: Query<&Transform, With<Rotates>>, mut rig_q: Query<&mut Rig>) {
    if let Ok(p) = query.get_single() {
        let mut rig = rig_q.single_mut();
        rig.driver_mut::<Position>().position = p.translation;
        rig.driver_mut::<Rotation>().rotation = p.rotation;
        rig.driver_mut::<LookAt>().target = p.translation + Vec3::Y;
    }
}

#[derive(Component)]
struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform =
            Transform::from_rotation(Quat::from_rotation_y((4.0 * PI / 20.0) * time.delta_secs()))
                * *transform;
    }
}

fn switch_camera_rig(
    camera: Res<State<Camera>>,
    mut next_camera: ResMut<NextState<Camera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        let result = if *camera == Camera::FollowPlayer {
            Camera::FollowSheep
        } else {
            Camera::FollowPlayer
        };

        println!("{result:?}");
        next_camera.set(result);
    }
}
