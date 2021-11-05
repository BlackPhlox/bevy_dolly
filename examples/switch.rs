mod helpers;
use bevy::prelude::*;
use bevy_dolly::prelude::*;

use helpers::*;

// In this example we are going to switch our look at target
// All you need to do is set a LookAt driver target_entity
// and its will track it

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyPlugin)
        .init_resource::<TargetConfig>()
        .add_startup_system(setup)
        .add_system(move_sheep_system)
        .add_system_to_stage(CoreStage::PreUpdate, change_target)
        .add_startup_system(setup_example_scene)
        .run();
}

// For now we will just save entities to a resource
// TODO: Add mouse click to select example
#[derive(Default)]
struct TargetConfig {
    entities: Vec<Entity>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut target_config: ResMut<TargetConfig>,
) {
    // Add a few more sheep and save the entities in our target config
    let mut herd = vec![
        helpers::spawn_sheep(Vec3::new(2.0, 0.2, 2.0), &mut commands, &asset_server),
        helpers::spawn_sheep(Vec3::new(2.0, 0.2, -2.0), &mut commands, &asset_server),
        helpers::spawn_sheep(Vec3::new(-2.0, 0.2, 2.0), &mut commands, &asset_server),
        helpers::spawn_sheep(Vec3::new(-2.0, 0.2, -2.0), &mut commands, &asset_server),
    ];
    target_config.entities.append(&mut herd);

    // Add our camera, try changing
    // You can remove the `Control` from the bundle to disable camera movement
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 2.0, -10.0),
            ..Default::default()
        })
        .insert(Rig::default().with(LookAt::new(
            target_config.entities[0],
            // Lets look a little in front of and above our target
            Vec3::new(0.0, 1.0, 1.0),
        )));

    info!(" Use 1, 2, 3, 4 to target different sheep");
    info!(" Use Q and E to turn the sheep");
}

// Look for key presses to select a target
fn change_target(
    target_config: Res<TargetConfig>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Rig>,
) {
    for mut rig in query.iter_mut() {
        if let Some(d) = rig.get_driver_mut::<LookAt>() {
            // Lets map keys to the targets, and set the target entity
            if keys.just_pressed(KeyCode::Key1) {
                d.target_entity = Some(target_config.entities[0]);
            }
            if keys.just_pressed(KeyCode::Key2) {
                d.target_entity = Some(target_config.entities[1]);
            }
            if keys.just_pressed(KeyCode::Key3) {
                d.target_entity = Some(target_config.entities[2]);
            }
            if keys.just_pressed(KeyCode::Key4) {
                d.target_entity = Some(target_config.entities[3]);
            }
        }
    }
}

/// Move Sheep around so we have something to track
fn move_sheep_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Sheep>>,
    mut left: Local<bool>,
) {
    if keys.just_pressed(KeyCode::Q) {
        *left = true;
    }
    if keys.just_pressed(KeyCode::E) {
        *left = false;
    }

    for mut sheep in query.iter_mut() {
        let movement = sheep.local_z() * 0.05;
        sheep.translation += movement;
        sheep.rotation *= Quat::from_rotation_y(if *left { 0.01 } else { -0.01 });
    }
}
