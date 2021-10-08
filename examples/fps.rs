use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_dolly::prelude::*;
mod helpers;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyPlugin)
        .add_startup_system(setup_camera)
        .add_system(update_camera_system)
        .add_startup_system(helpers::setup_example_scene)
        .run();
}

/// Set our cameras
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(DollyCameraBundle {
        rig_builder: RigBuilder::default()
            .add(Position::default())
            .add(Rotation::default())
            .add(YawPitch::new())
            .add(Smooth::new_position_rotation(1.0, 1.0)),
            //.add(Smooth::new_rotation(1.5)),
        transform: Transform::from_xyz(0.0, 2.0, -5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Print our user controls for reference
    info!("Use W, A, S, D for movement");
    info!("Use Shift to go fast");
}

// Lets handle input our selfs
fn update_camera_system(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Rig>,
) {
    for mut rig in query.iter_mut() {
        let time_delta_seconds: f32 = time.delta_seconds();
        let speed = 10.0;
        let boost_multiplyer: f32 = 5.0;
        let sensitivity = Vec2::splat(1.0);
        let mut move_vec = Vec3::ZERO;

        // Q: Is dolly left-handed so z is flipped?

        // Handle Input
        if keys.pressed(KeyCode::W) {
            move_vec.z -= 1.0;
        }
        if keys.pressed(KeyCode::S) {
            move_vec.z += 1.0;
        }
        if keys.pressed(KeyCode::A) {
            move_vec.x -= 1.0;
        }
        if keys.pressed(KeyCode::D) {
            move_vec.x += 1.0;
        }

        let boost: f32 = match keys.pressed(KeyCode::LShift) {
            true => boost_multiplyer,
            false => 1.0,
        };

        move_vec = rig.final_transform.rotation * move_vec.clamp_length_max(1.0) * boost;

        if let Some(d) = rig.get_driver_mut::<Position>() {
            d.position += move_vec * time_delta_seconds * speed;
        }


        // Update Mouse Movement
        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            delta += event.delta;
        }
        if let Some(d) = rig.get_driver_mut::<YawPitch>() {
            d.rotate_yaw_pitch(
                -0.1 * delta.x * sensitivity.x,
                -0.1 * delta.y * sensitivity.y,
            );
        }
    }
}
