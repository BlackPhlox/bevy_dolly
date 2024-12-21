use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use bevy_dolly::prelude::*;

#[derive(Component)]
struct MainCamera;

#[derive(States, Default, Clone, Debug, Eq, PartialEq, Hash)]
enum MovementType {
    #[default]
    FirstPerson,
    Free,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DollyCursorGrab))
        .init_state::<MovementType>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                Dolly::<MainCamera>::update_active,
                update_camera,
                update_fpvtype,
            ),
        )
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
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5., 5.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3)))
    ));

    let poly_dolly = asset_server.load(GltfAssetLabel::Scene(0).from_asset("poly_dolly.gltf"));

    commands.spawn((
        SceneRoot(poly_dolly),
        Transform::from_xyz(0., 0.2, 0.),
    ));

    let translation = [2.0f32, 2.0f32, 5.0f32];
    let transform =
        Transform::from_translation(Vec3::from_slice(&translation)).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn((
        MainCamera,
        Rig::builder()
            .with(Fpv::from_position_target(transform))
            .build(),
        Camera3d::default(),
        transform
    ));

    // light
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0)
    ));

    info!("Use W, A, S, D for movement");
    info!("Use Space/E and Ctrl/Q for going up and down");
    info!("Use Shift to go fast");
    info!("Use F to switch between Fps or Free camera");
    info!("Press Esc to toggle cursor focus");
}

fn update_fpvtype(
    keys: Res<ButtonInput<KeyCode>>,
    fps_state: Res<State<MovementType>>,
    mut fps_next_state: ResMut<NextState<MovementType>>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        let result = if *fps_state == MovementType::FirstPerson {
            MovementType::Free
        } else {
            MovementType::FirstPerson
        };

        println!("State:{result:?}");
        fps_next_state.set(result);
    }
}

fn update_camera(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    fps_state: Res<State<MovementType>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut rig_q: Query<&mut Rig>,
) {
    let time_delta_seconds: f32 = time.delta_secs();
    let boost_mult = 5.0f32;
    let sensitivity = Vec2::splat(1.0);

    let mut move_vec = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        move_vec.z -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        move_vec.z += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        move_vec.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        move_vec.x += 1.0;
    }

    if keys.pressed(KeyCode::KeyE) || keys.pressed(KeyCode::Space) {
        move_vec.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyQ) || keys.pressed(KeyCode::ControlLeft) {
        move_vec.y -= 1.0;
    }

    let boost: f32 = if keys.pressed(KeyCode::ShiftLeft) {
        boost_mult
    } else {
        1.
    };

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }
    delta.x *= sensitivity.x;
    delta.y *= sensitivity.y;

    let mut rig = rig_q.single_mut();

    if let Ok(window) = windows.get_single() {
        if !window.cursor_options.visible {
            rig.driver_mut::<Fpv>().update_pos_rot(
                move_vec,
                delta,
                *fps_state == MovementType::FirstPerson,
                boost,
                time_delta_seconds,
            );
        }
    }
}
