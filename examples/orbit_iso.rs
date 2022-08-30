use bevy::prelude::*;
use bevy::{input::mouse::MouseMotion, render::camera::ScalingMode};
use bevy_dolly::prelude::cursor_grab::DollyCursorGrab;
use bevy_dolly::prelude::*;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyCursorGrab)
        .add_dolly_component(MainCamera)
        .add_state(Pan::Keys)
        .add_startup_system(setup)
        .add_system(update_camera)
        .run();
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Pan {
    Mouse,
    Keys,
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
        ..default()
    });

    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("poly_dolly.gltf#Scene0"),
        transform: Transform {
            translation: Vec3::new(0., 0.2, 0.),
            ..default()
        },
        ..default()
    });

    commands
        .spawn()
        .insert(
            Rig::builder()
                .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
                .with(Smooth::new_rotation(1.5))
                .with(Arm::new(Vec3::Z * 4.0))
                .build(),
        )
        .insert(MainCamera);

    let camera = Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn_bundle(camera).insert(MainCamera);

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

#[allow(unused_must_use)]
fn update_camera(
    keys: Res<Input<KeyCode>>,
    mut pan: ResMut<State<Pan>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: ParamSet<(Query<(&mut Transform, With<MainCamera>)>, Query<&mut Rig>)>,
) {
    let mut p1 = query.p1();
    let mut rig = p1.single_mut();
    let camera_driver = rig.driver_mut::<YawPitch>();
    let sensitivity = Vec2::splat(2.0);

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }

    if pan.current().eq(&Pan::Keys) {
        if keys.just_pressed(KeyCode::Z) {
            camera_driver.rotate_yaw_pitch(-90.0, 0.0);
        }
        if keys.just_pressed(KeyCode::X) {
            camera_driver.rotate_yaw_pitch(90.0, 0.0);
        }
    } else {
        camera_driver.rotate_yaw_pitch(
            -0.1 * delta.x * sensitivity.x,
            -0.1 * delta.y * sensitivity.y,
        );
    }

    if keys.just_pressed(KeyCode::E) {
        let result = if pan.current().eq(&Pan::Keys) {
            Pan::Mouse
        } else {
            Pan::Keys
        };
        pan.overwrite_set(result);
        println!("State:{:?}", result);
    }
}
