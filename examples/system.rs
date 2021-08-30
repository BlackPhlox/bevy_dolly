use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_dolly::{
    system::{Cameras, Fps, MainCamera},
    DollyPlugins,
};
use dolly::prelude::CameraRig;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugins(DollyPlugins)
        .insert_resource(Cameras {
            cameras: vec![Box::new(Fps)],
        })
        .add_startup_system(setup.system())
        .add_system(update.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cams: Res<Cameras>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 2.0, 5.0)
                .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y),
            ..Default::default()
        })
        .insert(MainCamera);

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    cams.cameras[0].setup_camera(commands);
}

fn update(
    cams: Res<Cameras>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mouse_motion_events: EventReader<MouseMotion>,
    query: Query<(&mut Transform, With<MainCamera>)>,
    query2: Query<&mut CameraRig>,
) {
    cams.cameras[0].update_camera(time, keys, windows, mouse_motion_events, query, query2);
}
