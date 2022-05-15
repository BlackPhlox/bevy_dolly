use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_dolly::drivers::fps::Fps;
use bevy_dolly::prelude::*;

pub mod helpers;
use helpers::cursor_grab::DollyCursorGrab;

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
        });

    let translation = [-2.0f32, 2.0f32, 5.0f32];
    let transform = Transform::from_translation(bevy::math::Vec3::from_slice(&translation))
        .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y);

    commands.spawn().insert(
        CR::builder()
            .with(Fps::from_position_target(transform))
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

    info!("Use W, A, S, D for movement");
    info!("Use Shift to go fast");
}

fn update_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: ParamSet<(Query<(&mut Transform, With<MainCamera>)>, Query<&mut CR>)>,
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

    let mut p1 = query.p1();
    let mut rig = p1.single_mut();

    let move_vec = rig
        .driver_mut::<Fps>()
        .set_position(move_vec, boost, boost_mult, true);

    let window = windows.get_primary().unwrap();
    if window.cursor_locked() {
        rig.driver_mut::<Fps>()
            .set_rotation(delta, sensitivity, move_vec, time_delta_seconds);
    }

    let transform = rig.update(time_delta_seconds);
    let mut p0 = query.p0();
    let (mut cam, _) = p0.single_mut();

    cam.transform_2_bevy(transform);
}
