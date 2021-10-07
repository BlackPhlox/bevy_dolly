mod helpers;

use bevy::prelude::*;
use bevy_dolly::prelude::*;
use helpers::setup_example_scene;

// In this example we not going to use a bundle
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)

        .add_plugin(DollyPlugin)

        .add_startup_system(setup_example_scene)
        .add_startup_system(setup_camera)
        .add_system(update_camera_system)
        .run();
}

/// set up a simple 3D scene
fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn_bundle(DollyCameraBundle {
        camera_rig_builder: RigBuilder::default()
            .add(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
            .add(Smooth::new_rotation(1.5))
            .add(Arm { offset: Vec3::Z * 8.0 }),
        transform: Transform::from_xyz(0.0, 2.0, -5.0),
        ..Default::default()
    });
}

fn update_camera_system(
    mut query: Query<&mut CameraRig>,
    keys: Res<Input<KeyCode>>,
) {
    for mut rig in query.iter_mut() {
        if let Some(driver) = rig.get_driver_mut::<YawPitch>() {
            if keys.just_pressed(KeyCode::Z) {
                driver.rotate_yaw_pitch(-90.0, 0.0);
            }
            if keys.just_pressed(KeyCode::X) {
                driver.rotate_yaw_pitch(90.0, 0.0);
            }
        }
    }
}
