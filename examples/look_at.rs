#![allow(clippy::type_complexity)]
use bevy::prelude::*;
use bevy_dolly::prelude::*;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins((DefaultPlugins, DollyPosCtrl))
        .add_systems(Startup, setup)
        .add_systems(Update, (Dolly::<MainCamera>::update_active, update_camera))
        .run();
}
#[allow(dead_code)]
struct Player;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //mut config: ResMut<CtrlConfig>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });

    Transform::from_translation(Vec3::new(0., 0., 2.));

    /*
    config.entity = Some(
        commands
            .spawn_bundle((
                Transform {
                    translation: Vec3::new(0., 0.2, 0.),
                    ..default()
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

    commands.spawn((
        MainCamera,
        Rig::builder()
            .with(Position::new(Vec3::Y * 3.0))
            .with(LookAt::new(
                /*start_pos.transform_2_dolly().position*/
                Vec3::new(0., -2., 2.),
            ))
            .build(),
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2., 5.0)
                .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            ..default()
        },
    ));

    info!("Use Q and E to turn the sheep");
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn update_camera(
    mut query: ParamSet<(
        Query<&mut Transform, With<DollyPosCtrlMove>>,
        Query<&mut Rig>,
    )>,
) {
    let mut p0 = query.p0();
    let player = p0.single_mut();
    query.p1().single_mut().driver_mut::<LookAt>().target = player.translation;
}
