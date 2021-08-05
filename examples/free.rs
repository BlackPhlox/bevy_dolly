use bevy::app::Events;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use dolly::glam::Vec3;
use dolly::prelude::{Arm, CameraRig, Positional, Smooth, YawPitch};

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

    let start_pos = Vec3::new(0.,0.,0.);

    let camera = CameraRig::builder()
        .with(Positional::new(Vec3::Y))
        .with(YawPitch::new())
        .with(Smooth::new_move_look(1.0, 1.0))
        .build();

    commands.insert_resource(Dolly { rigs: camera });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 10.0, 5.0)
                .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y),
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
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut dolly: ResMut<Dolly>,
    mut query: Query<(&mut Transform, With<MainCamera>)>,
) {
    let (mut cam, _) = query.single_mut().unwrap();
    let time_delta_seconds : f32 = 0.1;
    let x_sensitivity : f32 = 1.;
    let y_sensitivity : f32 = 1.;

    let move_right: f32 = if keys.just_pressed(KeyCode::D) {
        0.
    } else {
        1.
    };
    let move_up: f32 = if keys.just_pressed(KeyCode::Space) {
        0.
    } else {
        1.
    };
    let move_fwd: f32 = if keys.just_pressed(KeyCode::W) {
        0.
    } else {
        1.
    };
    let boost: f32 = if keys.just_pressed(KeyCode::LShift) {
        0.
    } else {
        1.
    };

    let mut delta_x = 1.;
    let mut delta_y = 1.;
    for event in mouse_motion_events.iter() {
        delta_x = event.delta.x;
        delta_y = event.delta.y;
    }

    let move_vec = dolly.rigs.transform.rotation
        * Vec3::new(move_right, move_up, -move_fwd).clamp_length_max(1.0)
        * 10.0f32.powf(boost);

        
    dolly.rigs
        .driver_mut::<YawPitch>()
        .rotate_yaw_pitch(-0.1 * delta_x * x_sensitivity, -0.1 * delta_y * y_sensitivity);
        /*
    dolly.rigs
        .driver_mut::<Positional>()
        .translate(move_vec * time_delta_seconds * 10.0);
        */

    let transform = dolly.rigs.update(time_delta_seconds);
    let translation = transform.translation;
    let rotation = transform.rotation;

    cam.translation = bevy::math::Vec3::new(translation.x, translation.y, translation.z);
    cam.rotation = bevy::math::Quat::from_xyzw(rotation.x, rotation.y, rotation.z, rotation.w);
}
