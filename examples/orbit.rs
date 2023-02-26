use bevy::prelude::*;
use bevy::{input::mouse::MouseMotion, render::camera::ScalingMode};
use bevy_dolly::prelude::cursor_grab::DollyCursorGrab;
use bevy_dolly::prelude::*;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct SecondCamera;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyCursorGrab)
        .add_dolly_component(MainCamera)
        .add_state(Pan::Keys)
        .add_startup_system(setup)
        .add_system(update_camera)
        .add_system(swap_camera)
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

    commands.spawn((
        MainCamera,
        Rig::builder()
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
            .with(Smooth::new_rotation(1.5))
            .with(Arm::new(Vec3::Z * 4.0))
            .build(),
    ));

    let camera_iso = Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    let camera_perspective = Camera3dBundle {
        projection: PerspectiveProjection {
            ..Default::default()
        }
        .into(),
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            is_active: false,
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn(camera_iso).insert(MainCamera);
    commands.spawn(camera_perspective).insert(SecondCamera);

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    info!("Use Z and X to orbit the sheep");
    info!("Press E to toggle to use the mouse to orbit the sheep");
}

fn swap_camera(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut q_main: Query<(Entity, &mut Camera), (With<MainCamera>, Without<SecondCamera>)>,
    mut q_sec: Query<(Entity, &mut Camera), (With<SecondCamera>, Without<MainCamera>)>,
) {
    if keys.just_pressed(KeyCode::T) {
        if let Ok((e_main, cam_main)) = &mut q_main.get_single_mut() {
            if let Ok((e_sec, cam_sec)) = &mut q_sec.get_single_mut() {
                commands.entity(e_main.clone()).remove::<MainCamera>();
                commands.entity(e_sec.clone()).remove::<SecondCamera>();
                commands.entity(e_main.clone()).insert(SecondCamera);
                commands.entity(e_sec.clone()).insert(MainCamera);
                cam_sec.is_active = true;
                cam_main.is_active = false;
            }
        }
    }
}

#[allow(unused_must_use)]
fn update_camera(
    keys: Res<Input<KeyCode>>,
    mut pan: ResMut<State<Pan>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut rig_q: Query<&mut Rig>,
) {
    let mut rig = rig_q.single_mut();
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
