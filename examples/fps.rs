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
        .add_system(update_camera_system)
        .add_startup_system(setup_example_scene)
        .run();
}

/// Set our cameras
fn setup(mut commands: Commands) {
    commands.spawn_bundle(DollyCameraBundle {
        rig: Rig::default()
            .add(Position::default())
            .add(Rotation::default())
            .add(YawPitch::default())
            .add(Smooth::new(2.0, 2.0)),
        transform: Transform::from_xyz(0.0, 2.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Print our user controls for reference
    info!("Use W, A, S, D for movement");
    info!("Use Shift to go fast");
}

// Lets handle input ourselfs
fn update_camera_system(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&Transform, &mut Rig)>,
) {
    for (t, mut rig) in query.iter_mut() {
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

        // Move relative to the camera
        move_vec = t.rotation * move_vec.clamp_length_max(1.0) * boost;
        move_vec.y = 0.0; // clear out y so we don't move up and down

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
