use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use dolly::glam::Vec3;
use dolly::prelude::{CameraRig, Positional, Smooth, YawPitch};

struct Dolly {
    rigs: CameraRig,
}

struct MainCamera;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(update_camera.system())
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
            cell.spawn_scene(asset_server.load("sheep.gltf#Scene0"));
        })
        .id();

    let translation = [-2.0f32, 2.0f32, 5.0f32];
    let transform =
        Transform::from_translation(bevy::math::Vec3::from_slice_unaligned(&translation))
            .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y);
    let rotation = dolly::glam::Quat::from_xyzw(
        transform.rotation.x,
        transform.rotation.y,
        transform.rotation.z,
        transform.rotation.w,
    );
    let mut yaw_pitch = YawPitch::new();
    yaw_pitch.set_rotation(rotation);

    let camera = CameraRig::builder()
        .with(Positional {
            position: Vec3::from_slice(&translation),
            rotation,
        })
        .with(yaw_pitch)
        .with(Smooth::new_move_look(1.0, 1.0))
        .build();

    commands.insert_resource(Dolly { rigs: camera });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform,
            ..Default::default()
        })
        .insert(MainCamera);

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

/*
fn player_look(
    settings: Res<MovementSettings>,
    windows: Res<Windows>,
    motion: Res<Events<MouseMotion>>,
    mut state: ResMut<InputState>,
    mut query: Query<(&FlyCam, &mut Transform)>,
) {
    if settings.disable_look {
        return;
    }
    let window = windows.get_primary().unwrap();
    for (_camera, mut transform) in query.iter_mut() {
        for ev in state.reader_motion.iter(&motion) {
            if window.cursor_locked() {
                state.pitch -= (settings.sensitivity * ev.delta.y * window.height()).to_radians();
                state.yaw -= (settings.sensitivity * ev.delta.x * window.width()).to_radians();
            }

            state.pitch = state.pitch.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            transform.rotation = Quat::from_axis_angle(Vec3::Y, state.yaw)
                * Quat::from_axis_angle(Vec3::X, state.pitch);
        }
    }
}
*/

fn update_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut dolly: ResMut<Dolly>,
    mut query: Query<(&mut Transform, With<MainCamera>)>,
) {
    let (mut cam, _) = query.single_mut().unwrap();
    let time_delta_seconds: f32 = time.delta_seconds();
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

    let move_vec =
        dolly.rigs.transform.rotation * move_vec.clamp_length_max(1.0) * 10.0f32.powf(boost);

    dolly.rigs.driver_mut::<YawPitch>().rotate_yaw_pitch(
        -0.1 * delta.x * sensitivity.x,
        -0.1 * delta.y * sensitivity.y,
    );
    dolly
        .rigs
        .driver_mut::<Positional>()
        .translate(move_vec * time_delta_seconds * 10.0);

    let transform = dolly.rigs.update(time_delta_seconds);
    let translation = transform.translation;
    let rotation = transform.rotation;

    cam.translation = bevy::math::Vec3::new(translation.x, translation.y, translation.z);
    cam.rotation = bevy::math::Quat::from_xyzw(rotation.x, rotation.y, rotation.z, rotation.w);
}
