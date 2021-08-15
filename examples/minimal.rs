use bevy::prelude::*;
use bevy_dolly::Dolly;
use dolly::glam::Vec3;
use dolly::prelude::{Arm, CameraRig, Smooth, YawPitch};

struct Dolly2 {
    rigs: CameraRig,
}

struct MainCamera;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(Dolly)
        .add_startup_system(setup.system())
        .add_system(update_camera.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    let camera = CameraRig::builder()
        .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
        .with(Smooth::new_look(1.5))
        .with(Arm::new(Vec3::Z * 4.0))
        .build();

    commands.insert_resource(Dolly2 { rigs: camera });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 10.0, 5.0)
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
    keys: Res<Input<KeyCode>>,
    mut dolly: ResMut<Dolly2>,
    mut query: Query<(&mut Transform, With<MainCamera>)>,
) {
    let camera_driver = dolly.rigs.driver_mut::<YawPitch>();
    let time_delta_seconds: f32 = 0.1;

    if keys.just_pressed(KeyCode::Z) {
        camera_driver.rotate_yaw_pitch(-90.0, 0.0);
    }
    if keys.just_pressed(KeyCode::X) {
        camera_driver.rotate_yaw_pitch(90.0, 0.0);
    }

    let transform = dolly.rigs.update(time_delta_seconds);
    let (mut cam, _) = query.single_mut().unwrap();

    let (translation, rotation) = transform.into_translation_rotation();
    cam.translation = bevy::math::Vec3::new(translation.x, translation.y, translation.z);
    cam.rotation = bevy::math::Quat::from_xyzw(rotation.x, rotation.y, rotation.z, rotation.w);
}
