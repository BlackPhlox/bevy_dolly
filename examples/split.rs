#![allow(clippy::type_complexity)]
//! Renders two cameras to the same window to accomplish "split screen".

use bevy::{
    prelude::*,
    render::camera::Viewport,
    window::{PrimaryWindow, WindowResized},
};
use bevy_dolly::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                Dolly::<LeftCamera>::update_active,
                Dolly::<RightCamera>::update_active,
                set_camera_viewports,
                update_camera_1,
                update_camera_2,
            ),
        )
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
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100., 100.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3)))
    ));

    let poly_fox = asset_server.load(GltfAssetLabel::Scene(0).from_asset("poly_fox.glb"));

    commands.spawn((
        SceneRoot(poly_fox),
    ));

    let poly_dolly = asset_server.load(GltfAssetLabel::Scene(0).from_asset("poly_dolly.gltf"));

    commands.spawn((
        SceneRoot(poly_dolly),
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        }, 
        Transform::from_rotation(Quat::from_euler(
        EulerRot::ZYX,
        0.0,
        1.0,
        -std::f32::consts::FRAC_PI_4,
    ))));

    // camera
    commands.spawn((
        LeftCamera,
        Camera3d::default(),
        Transform::from_xyz(0.0, 200.0, -100.0).looking_at(Vec3::ZERO, Vec3::Y)
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
        .spawn((
            Camera3d::default(), 
            Camera {
                clear_color: ClearColorConfig::None,
                order: 1,
                ..default()
            },
            Transform::from_xyz(100.0, 100., 150.0).looking_at(Vec3::ZERO, Vec3::Y),
            RightCamera
        )
    );
}

fn set_camera_viewports(
    windows: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut resize_events: EventReader<WindowResized>,
    mut left_camera: Query<&mut Camera, (With<LeftCamera>, Without<RightCamera>)>,
    mut right_camera: Query<&mut Camera, With<RightCamera>>,
) {
    for resize_event in resize_events.read() {
        if let Ok((entity, window)) = windows.get_single() {
            if resize_event.window == entity {
                let mut left_camera = left_camera.single_mut();
                left_camera.viewport = Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(
                        window.physical_width() / 2,
                        window.physical_height(),
                    ),
                    depth: 0.0..1.0,
                });

                let mut right_camera = right_camera.single_mut();
                right_camera.viewport = Some(Viewport {
                    physical_position: UVec2::new(window.physical_width() / 2, 0),
                    physical_size: UVec2::new(
                        window.physical_width() / 2,
                        window.physical_height(),
                    ),
                    depth: 0.0..1.0,
                });
            }
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
    mut query: Query<&mut Rig, (With<RightCamera>, Without<LeftCamera>)>,
) {
    let mut rig = query.single_mut();
    let camera_driver = rig.driver_mut::<YawPitch>();

    camera_driver.rotate_yaw_pitch(-1.0, 0.0);

    let a = rig.driver_mut::<Arm>();
    a.offset = Vec3::Z * ((time.delta_secs() * 0.2).sin().cos().abs() * 400. - 200.);
}
