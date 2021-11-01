mod helpers;
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use helpers::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        // Add Dolly plugin
        .add_plugin(DollyPlugin)
        .add_startup_system(setup)
        .add_startup_system(setup_example_scene)
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands) {

    let camera =  DollyControlCameraBundle {
        rig: Rig::default(),
        transform: Transform::from_xyz(0.0, 2.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };
    // Now lets finally spawn our camera
    commands.spawn_bundle(camera);
}
