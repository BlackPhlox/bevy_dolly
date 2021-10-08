mod helpers;
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use helpers::setup_example_scene;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        
         // Add Dolly plugin
        .add_plugin(DollyPlugin)
        .add_startup_system(setup_camera)

        .add_startup_system(setup_example_scene)
        .run();
}

/// set up a simple 3D scene
fn setup_camera(mut commands: Commands) {
    // Create our camera with defaults, currently that is free look

    commands.spawn_bundle(DollyControlledCameraBundle::default());
}
