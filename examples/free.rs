use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_dolly::{DollyCursorGrab, UpdateMutTransform};
use dolly::prelude::{CameraRig, Position, Rotation, Smooth, YawPitch};

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyCursorGrab)
        .add_startup_system(setup)
        .add_system(update_camera)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    commands
        .spawn_bundle((
            Transform {
                translation: bevy::math::Vec3::new(0., 0.2, 0.),
                ..Default::default()
            },
            GlobalTransform::identity(),
        ))
        .with_children(|cell| {
            cell.spawn_scene(asset_server.load("poly_dolly.gltf#Scene0"));
        })
        .id();

    let translation = [-2.0f32, 2.0f32, 5.0f32];
    let transform = Transform::from_translation(bevy::math::Vec3::from_slice(&translation))
        .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y);

    let rotation = transform.rotation;
    let mut yaw_pitch = YawPitch::new();
    yaw_pitch.set_rotation_quat(rotation);

    commands.spawn().insert(
        CameraRig::builder()
            .with(Position {
                translation: Vec3::from_slice(&translation),
            })
            .with(Rotation { rotation })
            .with(yaw_pitch)
            .with(Smooth::new_position_rotation(1.0, 1.0))
            .build(),
    );

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform,
            ..Default::default()
        })
        .insert(MainCamera);

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn update_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: QuerySet<(
        QueryState<(&mut Transform, With<MainCamera>)>,
        QueryState<&mut CameraRig>,
    )>,
) {
    let time_delta_seconds: f32 = time.delta_seconds();
    let boost_mult = 5.0f32;
    let sensitivity = Vec2::splat(1.0);

    let mut move_vec = Vec3::ZERO;

    // Q: Is dolly left-handed so z is flipped?
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

    if keys.pressed(KeyCode::E) || keys.pressed(KeyCode::Space) {
        move_vec.y += 1.0;
    }
    if keys.pressed(KeyCode::Q) {
        move_vec.y -= 1.0;
    }

    let boost: f32 = if keys.pressed(KeyCode::LShift) {
        1.
    } else {
        0.
    };

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }

    let mut q1 = query.q1();
    let mut rig = q1.single_mut();

    let move_vec =
        rig.final_transform.rotation * move_vec.clamp_length_max(1.0) * boost_mult.powf(boost);

    let window = windows.get_primary().unwrap();
    if window.cursor_locked() {
        rig.driver_mut::<YawPitch>().rotate_yaw_pitch(
            -0.1 * delta.x * sensitivity.x,
            -0.1 * delta.y * sensitivity.y,
        );
        rig.driver_mut::<Position>()
            .translate(move_vec * time_delta_seconds * 10.0);
    }

    let transform = rig.update(time_delta_seconds);
    let mut q0 = query.q0();
    let (mut cam, _) = q0.single_mut();

    cam.update(transform);
}
