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


fn setup(mut commands: Commands) {

    // Create our camera with defaults, currently that is free look
    let camera =  DollyControlCameraBundle {
        rig: Rig::default()
            .add(Position::default())
            .add(Rotation::default())
            .add(YawPitch::default())
            .add(Smooth::new(2.0, 2.0)),
        transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    // You could just pass the bundle directly to spawn_bundle
    // but lets print the current camera actions for ref
    camera.control_actions.print_actions();

    // Now lets finally spawn our camera
    commands.spawn_bundle(camera);

}
