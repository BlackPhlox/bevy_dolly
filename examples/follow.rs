use bevy::prelude::*;
use bevy_dolly::*;
#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate_sheep_system)
        .add_system(update_camera_system)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 1., 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(
                // TODO: can we have two smooths?
            CameraRig::builder()
                .with(Position::new(start_pos))
                .with(Rotation::new(Quat::IDENTITY))
                .with(Smooth::new_position(1.25).predictive(true))
                .with(Arm::new(Vec3::new(0.0, 1.5, -3.5)))
                .with(Smooth::new_position(2.5))
                .with(
                    LookAt::new(start_pos + Vec3::Y)
                        .tracking_smoothness(1.25)
                        .tracking_predictive(true),
                )
                .build(),
        );
}

/// Rotates Sheep
fn rotate_sheep_system(time: Res<Time>, mut query: Query<&mut Transform, With<Sheep>>) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_rotation_y((4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds());
    }
}
