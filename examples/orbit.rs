#![allow(clippy::type_complexity)]
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_dolly::prelude::*;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct SecondCamera;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyPosCtrl)
        .add_plugin(DollyCursorGrab)
        .insert_resource(DollyPosCtrlConfig {
            ..Default::default()
        })
        .add_state::<ProjectionType>()
        .add_state::<Pan>()
        .add_state::<ZoomType>()
        .add_system(Dolly::<MainCamera>::update_active)
        .add_startup_system(setup)
        .add_system(update_camera)
        .add_system(swap_camera)
        .add_system(handle_mouse_scroll)
        .run();
}

#[derive(States, Default, PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum ProjectionType {
    #[default]
    Orthographic,
    Perspective,
}

#[derive(States, Default, PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Pan {
    #[default]
    Mouse,
    Keys,
}

#[derive(States, Default, PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum ZoomType {
    #[default]
    Arm,
    Fov,
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    startup_perspective: Res<State<ProjectionType>>,
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
    commands.spawn(Transform::from_xyz(0., 0., 0.));

    commands
        .spawn(SceneBundle {
            scene: asset_server.load("poly_dolly.gltf#Scene0"),
            transform: Transform {
                translation: Vec3::new(0., 0.2, 0.),
                ..default()
            },
            ..default()
        })
        .insert(DollyPosCtrlMove);

    commands.spawn((
        MainCamera,
        Rig::builder()
            .with(Position::new(Vec3::ZERO))
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
            .with(Smooth::new_position(0.3))
            .with(Smooth::new_rotation(0.3))
            .with(Arm::new(Vec3::Z * 4.0))
            .build(),
    ));

    let orth = Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    let pers = Camera3dBundle {
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

    if startup_perspective.0 == ProjectionType::Orthographic {
        commands.spawn((MainCamera, orth));
        commands.spawn((SecondCamera, pers));
    } else {
        commands.spawn((MainCamera, pers));
        commands.spawn((SecondCamera, orth));
    }

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    info!("Use W, A, S, D for movement");
    info!("Use Space and Shift for going up and down");
    info!("Use Z and X to orbit the sheep");
    info!("Or press E to toggle to use the mouse to orbit the sheep");
    info!("Press T to toggle between orthographic and perspective camera");
    info!("Scroll to Zoom (Press G to switch between changing arm length and fov for perspective and scale for orthographic)");
    info!("Press P to toggle pinned to entity with DollyPosCtrlMove component");
    info!("Press Esc to toggle cursor focus");
}

fn swap_camera(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut perspective: ResMut<State<ProjectionType>>,
    mut zoom: ResMut<State<ZoomType>>,
    mut q_main: Query<(Entity, &mut Camera), (With<MainCamera>, Without<SecondCamera>)>,
    mut q_sec: Query<(Entity, &mut Camera), (With<SecondCamera>, Without<MainCamera>)>,
) {
    if keys.just_pressed(KeyCode::T) {
        if let Ok((e_main, cam_main)) = &mut q_main.get_single_mut() {
            if let Ok((e_sec, cam_sec)) = &mut q_sec.get_single_mut() {
                commands
                    .entity(*e_main)
                    .remove::<MainCamera>()
                    .insert(SecondCamera);
                commands
                    .entity(*e_sec)
                    .remove::<SecondCamera>()
                    .insert(MainCamera);
                cam_sec.is_active = true;
                cam_main.is_active = false;

                perspective.0 = if perspective.0 == ProjectionType::Orthographic {
                    ProjectionType::Perspective
                } else {
                    ProjectionType::Orthographic
                };

                println!("Perspective: {:?}", perspective.0);
            }
        }
    } else if keys.just_pressed(KeyCode::G) {
        // Arm doesn't make a difference for Orthographic projection
        zoom.0 = if zoom.0 == ZoomType::Arm && perspective.0 == ProjectionType::Perspective {
            ZoomType::Fov
        } else {
            ZoomType::Arm
        };
        println!("ZoomType: {:?}", zoom.0);
    }
}

fn handle_mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut q_main: Query<&mut Projection, With<MainCamera>>,
    zoom: Res<State<ZoomType>>,
    mut rig_q: Query<&mut Rig>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for mut projection in &mut q_main.iter_mut() {
            match &mut projection.as_mut() {
                Projection::Perspective(pers) => {
                    if zoom.0 == ZoomType::Fov {
                        pers.fov = (pers.fov - mouse_wheel_event.y * 0.01).abs();
                    } else {
                        if let Ok(mut rig) = rig_q.get_single_mut() {
                            if let Some(arm) = rig.try_driver_mut::<Arm>() {
                                let mut xz = arm.offset;
                                xz.z = (xz.z - mouse_wheel_event.y * 0.5).abs();
                                arm.offset = xz;
                            }
                        }
                    }
                }
                Projection::Orthographic(orth) => {
                    orth.scale = (orth.scale - mouse_wheel_event.y * 0.1).abs();
                }
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
    trans: Query<&Transform, With<DollyPosCtrlMove>>,
    mut config: ResMut<DollyPosCtrlConfig>,
    grab_config: Res<DollyCursorGrabConfig>,
) {
    let mut rig = rig_q.single_mut();
    let camera_yp = rig.driver_mut::<YawPitch>();
    let sensitivity = Vec2::splat(2.0);

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }

    config.rotation = Quat::from_rotation_y(delta.x);

    if pan.0.eq(&Pan::Keys) {
        if keys.just_pressed(KeyCode::Z) {
            camera_yp.rotate_yaw_pitch(-90.0, 0.0);
        }
        if keys.just_pressed(KeyCode::X) {
            camera_yp.rotate_yaw_pitch(90.0, 0.0);
        }
    } else if !grab_config.visible {
        camera_yp.rotate_yaw_pitch(
            -0.1 * delta.x * sensitivity.x,
            -0.1 * delta.y * sensitivity.y,
        );
    }

    if keys.just_pressed(KeyCode::E) {
        let result = if pan.0.eq(&Pan::Keys) {
            Pan::Mouse
        } else {
            Pan::Keys
        };
        pan.0 = result;
        println!("PanType: {result:?}");
    }

    if keys.just_pressed(KeyCode::P) {
        config.pin = !config.pin;
        println!(
            "Camera: {}",
            match config.pin {
                true => "Pinned",
                false => "Unpinned",
            }
        );
    }

    if config.pin {
        if let Some(camera_pos) = rig.try_driver_mut::<Position>() {
            if let Ok(pos) = trans.get_single() {
                // + Offset
                camera_pos.position = pos.translation + Vec3::new(0., 1., 0.);
            }
        }
    }
}
