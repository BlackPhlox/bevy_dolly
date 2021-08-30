use bevy::prelude::*;
use bevy_dolly::ctrl::{DollyCtrlMove, DollyDefaultCtrlConfig};
use bevy_dolly::{DollyPlugins, Transform2Bevy, Transform2Dolly};
use dolly::glam::Vec3;
use dolly::prelude::{CameraRig, LookAt, Position};

struct MainCamera;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugins(DollyPlugins)
        .add_startup_system(setup.system())
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
    mut config: ResMut<DollyDefaultCtrlConfig>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    let start_pos = Transform::from_translation(bevy::math::Vec3::new(0., 0., 2.));

    /*
    config.entity = Some(
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
            .insert(Player)
            .id(),
    );
    */

    commands.spawn().insert(
        CameraRig::builder()
            .with(Position::new(Vec3::Y * 3.0))
            .with(LookAt::new(
                /*start_pos.transform_2_dolly().position*/
                dolly::glam::Vec3::new(0., 0., 2.),
            ))
            .build(),
    );

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 1., 2.0).looking_at(
                /*start_pos.translation*/ bevy::math::Vec3::new(0., 0., 0.),
                bevy::math::Vec3::Y,
            ),
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
        Query<(&mut Transform, With<DollyCtrlMove>)>,
        Query<&mut CameraRig>,
    )>,
) {
    let (player, _) = query.q1_mut().single_mut().unwrap();
    query
        .q2_mut()
        .single_mut()
        .unwrap()
        .driver_mut::<LookAt>()
        .target = player.transform_2_dolly().position;

    let transform = query
        .q2_mut()
        .single_mut()
        .unwrap()
        .update(time.delta_seconds());
    let (mut cam, _) = query.q0_mut().single_mut().unwrap();

    cam.transform_2_bevy(transform);
}
