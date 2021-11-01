mod helpers;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use helpers::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyPlugin)
        .add_startup_system(setup)
        .add_startup_system(setup_example_scene)
        .run();
}

/// Set our cameras
fn setup(mut commands: Commands) {
    commands.spawn_bundle(DollyControlCameraBundle {
        rig: Rig::default(),
        transform: Transform::from_xyz(0.0, 2.0, -5.0)
        .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Print our user controls for reference
    info!("Use W, A, S, D for movement");
    info!("Use Shift to go fast");
}