mod helpers;
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
        .add_system(move_sheep_system)
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create another sheep, this time we will add a component to it so we can move it in a system
    let sheep = commands
        .spawn_bundle((Transform::from_xyz(2., 0.2, 0.), GlobalTransform::default()))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("poly_dolly.gltf#Scene0"));
        })
        .insert(Sheep)
        .id();

    commands.spawn_bundle(DollyCameraBundle {
        rig: Rig::default()
            .add(Anchor::new(sheep))
            .add(Arm::new(Vec3::new(0.0, 2.5, -3.5)))
            .add(LookAt::new(sheep, Vec3::new(0.0, 1.0, 1.0)))
            .add(Smooth::new(1.0, 1.0)),
        ..Default::default()
    });

    info!(" Use Z and X to turn the sheep");
}

/// Move Sheep around so we have something to
fn move_sheep_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Sheep>>,
    mut left: Local<bool>,
) {
    if keys.just_pressed(KeyCode::Z) {
        *left = true;
    }
    if keys.just_pressed(KeyCode::X) {
        *left = false;
    }

    for mut sheep in query.iter_mut() {
        let rotation = sheep.rotation.clone();
        sheep.translation += rotation * (Vec3::Z * 0.05);
        sheep.rotation *= Quat::from_rotation_y(if *left { 0.01 } else { -0.01 });
    }
}
