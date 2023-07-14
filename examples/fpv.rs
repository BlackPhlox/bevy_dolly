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
        .insert_resource(Msaa::default())
        .add_plugins((DefaultPlugins, DollyCursorGrab))
        .add_state::<MovementType>()
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
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            ..Default::default()
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("poly_dolly.gltf#Scene0"),
        transform: Transform {
            translation: Vec3::new(0., 0.2, 0.),
            ..default()
        },
        ..default()
    });

    let translation = [2.0f32, 2.0f32, 5.0f32];
    let transform =
        Transform::from_translation(Vec3::from_slice(&translation)).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn((
        MainCamera,
        Rig::builder()
            .with(Fpv::from_position_target(transform))
            .build(),
        Camera3dBundle {
            transform,
            ..default()
        },
    ));

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    info!("Use W, A, S, D for movement");
    info!("Use Shift to go fast");
    info!("Use F to switch between Fps or Free camera");
    info!("Press Esc to toggle cursor focus");
}

fn update_fpvtype(
    keys: Res<Input<KeyCode>>,
    fps_state: Res<State<MovementType>>,
    mut fps_next_state: ResMut<NextState<MovementType>>,
) {
    if keys.just_pressed(KeyCode::F) {
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
    keys: Res<Input<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    fps_state: Res<State<MovementType>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut rig_q: Query<&mut Rig>,
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

    let boost: f32 = if keys.pressed(KeyCode::ShiftLeft) {
        boost_mult
    } else {
        1.
    };

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }
    delta.x *= sensitivity.x;
    delta.y *= sensitivity.y;

    let mut rig = rig_q.single_mut();

    if let Ok(window) = windows.get_single() {
        if !window.cursor.visible {
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
