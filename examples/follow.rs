use bevy::prelude::*;
use bevy_dolly::prelude::*;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotator_system)
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

    let start_pos = Vec3::new(0., 0., 0.);

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
        .insert(Rotates);

    commands.spawn().insert(
        CR::builder()
            .with(MovableLookAt::from_position_target(start_pos))
            .build(),
    );

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 1., 5.0)
                .looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y),
            ..Default::default()
        })
        .insert(MainCamera);

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn update_camera(
    time: Res<Time>,
    mut query: ParamSet<(
        Query<(&mut Transform, With<MainCamera>)>,
        Query<(&Transform, With<Rotates>)>,
        Query<&mut CR>,
    )>,
) {
    let p1 = query.p1();
    let player = p1.single().0.to_owned();

    let mut p2 = query.p2();
    let mut rig = p2.single_mut();

    rig.driver_mut::<MovableLookAt>()
        .set_position_target(player.translation, player.rotation);

    let transform = rig.update(time.delta_seconds());

    query.p0().single_mut().0.transform_2_bevy(transform);
}

#[derive(Component)]
struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(bevy::math::Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}

/// A custom camera rig which combines smoothed movement with a look-at driver.
#[derive(Debug, Deref, DerefMut)]
pub struct MovableLookAt(CameraRig<RightHanded>);

// Turn the nested rig into a driver, so it can be used in another rig.
impl RigDriver<RightHanded> for MovableLookAt {
    fn update(
        &mut self,
        params: dolly::rig::RigUpdateParams<RightHanded>,
    ) -> dolly::transform::Transform<RightHanded> {
        self.0.update(params.delta_time_seconds)
    }
}

impl MovableLookAt {
    pub fn from_position_target(target_position: dolly::glam::Vec3) -> Self {
        Self(
            CameraRig::builder()
                .with(Position::new(target_position))
                .with(Rotation::new(Quat::IDENTITY))
                .with(Smooth::new_position(1.25).predictive(true))
                .with(Arm::new(Vec3::new(0.0, 1.5, -3.5)))
                .with(Smooth::new_position(2.5))
                .with(
                    LookAt::new(target_position + Vec3::Y)
                        .tracking_smoothness(1.25)
                        .tracking_predictive(true),
                )
                .build(),
        )
    }

    pub fn set_position_target(
        &mut self,
        target_position: dolly::glam::Vec3,
        target_rotation: dolly::glam::Quat,
    ) {
        self.driver_mut::<Position>().position = target_position;
        self.driver_mut::<Rotation>().rotation = target_rotation;
        self.driver_mut::<LookAt>().target = target_position + Vec3::Y;
    }
}
