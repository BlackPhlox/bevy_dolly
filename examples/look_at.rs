use bevy::prelude::*;
use bevy_dolly::*;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyPlugin)
        .add_startup_system(setup)
        .add_system(update_camera_system)
        .run();
}

#[derive(Component)]
struct Player;

// TODO: Come back and fix this example
/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    let _start_pos = Transform::from_translation(bevy::math::Vec3::new(0., 0., 2.));

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
                Vec3::new(0., 0., 2.),
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
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
#[allow(clippy::type_complexity)]
fn update_camera_system(
    time: Res<Time>,
    mut query: QuerySet<(
        QueryState<&mut Transform, With<MainCamera>>,
        QueryState<&mut Transform, With<CtrlMove>>,
        QueryState<&mut CameraRig>,
    )>,
) {
    let mut q1 = query.q1();
    let player = q1.single_mut();
    query.q2().single_mut().driver_mut::<LookAt>().target = player.translation;

    let transform = query.q2().single_mut().update(time.delta_seconds());

    let mut q0 = query.q0();
    let mut cam = q0.single_mut();

    *cam = transform;
}