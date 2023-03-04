//! Renders two cameras to the same window to accomplish "split screen".

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::camera::Viewport,
    window::{WindowId, WindowResized},
};
use bevy_dolly::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_dolly_component(LeftCamera)
        .add_dolly_component(RightCamera)
        .add_startup_system(setup)
        .add_system(set_camera_viewports)
        .add_system(update_camera_1)
        .add_system(update_camera_2)
        .run();
}

#[derive(Component)]
struct LeftCamera;

#[derive(Component)]
struct RightCamera;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("poly_fox.glb#Scene0"),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("poly_dolly.gltf#Scene0"),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        )),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // camera
    commands.spawn((
        LeftCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 200.0, -100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    commands.spawn((
        LeftCamera,
        Rig::builder()
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
            .with(Smooth::new_rotation(1.5))
            .with(Arm::new(Vec3::Z * 4.0))
            .build(),
    ));

    commands.spawn((
        RightCamera,
        Rig::builder()
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
            .with(Smooth::new_rotation(1.5))
            .with(Arm::new(Vec3::Z * 200.0))
            .build(),
    ));

    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(100.0, 100., 150.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera: Camera {
                priority: 1,
                ..default()
            },
            ..default()
        })
        .insert(RightCamera);
}

fn set_camera_viewports(
    windows: Res<Windows>,
    mut resize_events: EventReader<WindowResized>,
    mut left_camera: Query<&mut Camera, (With<LeftCamera>, Without<RightCamera>)>,
    mut right_camera: Query<&mut Camera, With<RightCamera>>,
) {
    for resize_event in resize_events.iter() {
        if resize_event.id == WindowId::primary() {
            let window = windows.primary();
            let mut left_camera = left_camera.single_mut();
            left_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(window.physical_width() / 2, window.physical_height()),
                depth: 0.0..1.0,
            });

            let mut right_camera = right_camera.single_mut();
            right_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(window.physical_width() / 2, 0),
                physical_size: UVec2::new(window.physical_width() / 2, window.physical_height()),
                depth: 0.0..1.0,
            });
        }
    }
}

fn update_camera_1(mut query: Query<&mut Rig, (With<LeftCamera>, Without<RightCamera>)>) {
    let mut rig = query.single_mut();
    let camera_driver = rig.driver_mut::<YawPitch>();

    camera_driver.rotate_yaw_pitch(1.0, 0.0);
}

fn update_camera_2(
    time: Res<Time>,
    mut query: Query<(&mut Rig, (With<RightCamera>, Without<LeftCamera>))>,
) {
    let (mut rig, _a) = query.single_mut();
    let camera_driver = rig.driver_mut::<YawPitch>();

    camera_driver.rotate_yaw_pitch(-1.0, 0.0);

    let a = rig.driver_mut::<Arm>();
    a.offset = Vec3::Z * ((time.delta_seconds() as f32 * 0.2).sin().cos().abs() * 400. - 200.);
}
