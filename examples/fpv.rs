use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_dolly::prelude::*;

#[derive(Component)]
struct MainCamera;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum MovementType {
    FirstPerson,
    Free,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyCursorGrab)
        .add_state(MovementType::FirstPerson)
        .add_dolly_component(MainCamera)
        .add_startup_system(setup)
        .add_system(update_fpvtype)
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
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
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

    let translation = [-2.0f32, 2.0f32, 5.0f32];
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

fn update_fpvtype(keys: Res<Input<KeyCode>>, mut fps_state: ResMut<State<MovementType>>) {
    if keys.just_pressed(KeyCode::F) {
        let result = if fps_state.current().eq(&MovementType::FirstPerson) {
            MovementType::Free
        } else {
            MovementType::FirstPerson
        };

        println!("State:{:?}", result);
        let _ = fps_state.overwrite_set(result);
    }
}

fn update_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
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

    let boost: f32 = if keys.pressed(KeyCode::LShift) {
        1.
    } else {
        0.
    };

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }

    let mut rig = rig_q.single_mut();

    let move_vec = rig.driver_mut::<Fpv>().set_position(
        move_vec,
        boost,
        boost_mult,
        fps_state.current().eq(&MovementType::FirstPerson),
    );

    let window = windows.get_primary();

    if window.is_some() && !window.unwrap().cursor_visible() {
        rig.driver_mut::<Fpv>()
            .set_rotation(delta, sensitivity, move_vec, time_delta_seconds);
    }
}
