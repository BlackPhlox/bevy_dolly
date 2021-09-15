use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_dolly::drivers::fps::Vec3KeyMapWithBoost;
use bevy_dolly::{cam_ctrl::DollyCursorGrab, Transform2Bevy, Transform2Dolly};
use bevy_dolly::{IterAnyPressed, SubRig, SubRigBuild, WithRigSettings, ZeroYRotation};
use dolly::glam::Vec3;
use dolly::prelude::{CameraRig, Position, Rotation, Smooth, YawPitch};

struct MainCamera;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyCursorGrab)
        .add_startup_system(setup.system())
        .add_system(update_camera.system())
        .run();
}

#[derive(Debug)]
struct FpsTest {
    rig: CameraRig,
}

impl FpsTest {
    pub fn custom_update(
        &mut self,
        time: Res<Time>,
        keys: Res<Input<KeyCode>>,
        windows: Res<bevy::window::Windows>,
        mut mouse_motion_events: EventReader<MouseMotion>,
        sensitivity: Vec2,
        map: Vec3KeyMapWithBoost,
    ) {
        let time_delta_seconds: f32 = time.delta_seconds();
        let mut move_vec = Vec3::ZERO;
        let mut delta = Vec2::ZERO;
        let boost_mult = 5.0f32;
        let mut boost = 0.0;

        // Q: Is dolly left-handed so z is flipped?
        for key in keys.get_pressed() {
            if map.forward.is_being_pressed(key) {
                move_vec.z -= 1.0;
            }
            if map.backward.is_being_pressed(key) {
                move_vec.z += 1.0;
            }
            if map.left.is_being_pressed(key) {
                move_vec.x -= 1.0;
            }
            if map.right.is_being_pressed(key) {
                move_vec.x += 1.0;
            }

            if map.up.is_being_pressed(key) {
                move_vec.y += 1.0;
            }
            if map.down.is_being_pressed(key) {
                move_vec.y -= 1.0;
            }

            boost = if map.boost.is_being_pressed(key) {
                1.
            } else {
                0.
            };
        }

        for event in mouse_motion_events.iter() {
            delta += event.delta;
        }

        let move_vec = self.rig.final_transform.rotation.zero_y_rotation()
            * move_vec.clamp_length_max(1.0)
            * boost_mult.powf(boost);

        let window = windows.get_primary().unwrap();
        if window.cursor_locked() {
            self.rig.driver_mut::<YawPitch>().rotate_yaw_pitch(
                -0.1 * delta.x * sensitivity.x,
                -0.1 * delta.y * sensitivity.y,
            );

            self.rig
                .driver_mut::<Position>()
                .translate(move_vec * time_delta_seconds * 10.0);
        }
    }
}

struct FpsTestSettings {
    transform: dolly::transform::Transform,
}

impl WithRigSettings<FpsTestSettings> for FpsTest {
    fn init(settings: FpsTestSettings) -> Self {
        let mut yp = YawPitch::new();
        yp.set_rotation_quat(settings.transform.rotation);
        FpsTest {
            rig: CameraRig::builder()
                .with(Position {
                    position: settings.transform.position,
                })
                .with(Rotation {
                    rotation: settings.transform.rotation,
                })
                .with(yp)
                .with(Smooth::new_position_rotation(1.0, 0.5))
                .build(),
        }
    }
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
        })
        .id();

    let translation = [-2.0f32, 2.0f32, 5.0f32];
    let transform =
        Transform::from_translation(bevy::math::Vec3::from_slice_unaligned(&translation))
            .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y);

    let rotation = transform.transform_2_dolly().rotation;

    commands.spawn().insert(
        CameraRig::builder()
            .with_sub_rig::<FpsTest>(FpsTestSettings {
                transform: dolly::transform::Transform {
                    position: Vec3::from_slice(&translation),
                    rotation,
                },
            })
            .build(),
    );

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform,
            ..Default::default()
        })
        .insert(MainCamera);

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn update_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mouse_motion_events: EventReader<MouseMotion>,
    mut query: QuerySet<(
        Query<(&mut Transform, With<MainCamera>)>,
        Query<&mut CameraRig>,
    )>,
) {
    let time_delta_seconds: f32 = time.delta_seconds();
    let sensitivity = Vec2::splat(1.0);

    let mut rig = query.q1_mut().single_mut().unwrap();
    rig.driver_mut::<SubRig<FpsTest>>().driver.custom_update(
        time,
        keys,
        windows,
        mouse_motion_events,
        sensitivity,
        Vec3KeyMapWithBoost::default(),
    );

    let transform = rig.update(time_delta_seconds);
    let (mut cam, _) = query.q0_mut().single_mut().unwrap();

    cam.transform_2_bevy(transform);
}
