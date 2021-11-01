mod helpers;

use bevy::prelude::*;
use bevy_dolly::prelude::*;
use helpers::*;

// In this example we not going to use a bundle
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyPlugin)
        .add_startup_system(setup_camera)
        .add_system(update_camera_system)
        .add_startup_system(setup_example_scene)
        .run();
}

/// Set our cameras
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(DollyCameraBundle {
        rig: Rig::default()
            .with(Smooth::new(0.0, 1.5))
            .with(Arm::new(Vec3::Z * 8.0)),
        transform: Transform::from_xyz(0.0, 2.0, -5.0),
        ..Default::default()
    });
    info!("Use Z and X to rotate");
}

/// Rotate our camera around
fn update_camera_system(mut query: Query<&mut Rig>, keys: Res<Input<KeyCode>>) {
    for mut rig in query.iter_mut() {
        if keys.just_pressed(KeyCode::Z) {
            rig.target.rotate(Quat::from_rotation_x(-90.0));
        }
        if keys.just_pressed(KeyCode::X) {
            rig.target.rotate(Quat::from_rotation_x(90.0));
        }
    }
}
