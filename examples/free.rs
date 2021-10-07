mod helpers;

use bevy::prelude::*;
use bevy_dolly::prelude::*;
use helpers::spawn_example_scene;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)

        // Add Dolly plugin
        .add_plugin(DollyPlugin)

        .add_startup_system(spawn_example_scene)
        .add_startup_system(spawn_camera)
        .run();
}

/// set up a simple 3D scene
fn spawn_camera(mut commands: Commands) {

    // How you can spawn a normal camera
    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_xyz(0.0, 2.0, -5.0)
    //     // Look at something
    //         .looking_at( Vec3::ZERO, Vec3::Y),
    //     ..Default::default()
    // });

    // Create our camera with defaults, currently that is free look
    commands.spawn_bundle(DollyCameraBundle {
         // We can we create it anywhere we want
         transform: Transform::from_xyz(0.0, 2.0, -5.0)
        // Look at something
            .looking_at( Vec3::ZERO, Vec3::Y),
         ..Default::default()
     });
}
