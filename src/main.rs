use bevy::prelude::*;
use dolly::glam::Vec3;
use dolly::prelude::{Arm, CameraRig, Smooth, YawPitch};

struct Dolly{
    camera: CameraRig
}


impl Default for Dolly {
    fn default() -> Self {
        Dolly{camera: CameraRig{drivers: vec![], transform: dolly::transform::Transform::IDENTITY }}
    }
}


struct MainCamera;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(update_camera.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut rig : ResMut<Dolly>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        }).id();
    
    let camera = CameraRig::builder()
        .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
        .with(Smooth::new_look(1.5))
        .with(Arm::new(Vec3::Z * 4.0))
        .build();

    commands.spawn().insert(Dolly{camera});

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 10.0, 5.0).looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y),
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
    mut rig_opt : ResMut<Option<Dolly>>,
    mut query: Query<(&mut Transform, With<MainCamera>)>,
) {
    if rig_opt.is_none() {return};
    let (mut cam, _) = query.single_mut().unwrap();
    let camera_driver = rig_opt.unwrap().camera.driver_mut::<YawPitch>();
    if keys.just_pressed(KeyCode::Z) {
        camera_driver.rotate_yaw_pitch(-90.0, 0.0);
    }
    if keys.just_pressed(KeyCode::X) {
        camera_driver.rotate_yaw_pitch(90.0, 0.0);
    }

    let transform = rig_opt.unwrap().camera.update(0.1);
    let translation = transform.translation;
    let rotation = transform.rotation;

    cam.translation = bevy::math::Vec3::new(translation.x, translation.y, translation.z);
    cam.rotation = bevy::math::Quat::from_xyzw(rotation.x, rotation.y, rotation.z, rotation.w);
}