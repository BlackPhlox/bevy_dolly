use bevy::prelude::*;
use bevy_dolly::prelude::*;

#[derive(Component)]
struct MainCamera;

const SPEED: f32 = 4.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        //If large amount of smoothing is used, where camera movement is expected beyond the time of input
        //Ie. motion smoothing beyond 0.25, use update_2d_active_continuous instead
        .add_systems(
            Update,
            (Dolly::<MainCamera>::update_2d_active, update_camera),
        )
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let offset_transform = Transform::from_xyz(100., 0., 0.);

    let mut dolly = Sprite::from_image(asset_server.load("bevy_dolly.png"));
    dolly.custom_size = Some(Vec2::new(128., 128.));
    commands.spawn((dolly, offset_transform.with_translation(Vec3 { x: 0., y: 0., z: 1. })));

    commands.spawn((
        Camera2d::default(),
        MainCamera,
        Rig::builder()
            .with(Position::default())
            .with(Smooth::new_position(0.25))
            .build(),
    ));

    let mut room = Sprite::from_image(asset_server.load("room.png"));
    room.custom_size = Some(Vec2::new(2.6 * 800., 800.));

    commands.spawn((room, offset_transform));

    info!("Use W, A, S, D for movement");
    info!("Use Z & X zooming in and out");
}

fn update_camera(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Rig>) {
    for mut rig in &mut query {
        for &key in keys.get_pressed() {
            let pos_driver = rig.try_driver_mut::<Position>();
            if let Some(pos) = pos_driver {
                if key == KeyCode::KeyW {
                    pos.translate(SPEED * Vec3::Y);
                }
                if key == KeyCode::KeyA {
                    pos.translate(SPEED * -Vec3::X);
                }
                if key == KeyCode::KeyS {
                    pos.translate(SPEED * -Vec3::Y);
                }
                if key == KeyCode::KeyD {
                    pos.translate(SPEED * Vec3::X);
                }
                if key == KeyCode::KeyZ {
                    pos.translate(SPEED * -Vec3::Z);
                }
                if key == KeyCode::KeyX {
                    pos.translate(SPEED * Vec3::Z);
                }
            }

            let smooth_driver = rig.try_driver_mut::<Smooth>();
            if let Some(smooth) = smooth_driver {
                if key == KeyCode::KeyC {
                    smooth.position_smoothness = (smooth.position_smoothness - 0.001).abs();
                    println!("Smoothness {}", smooth.position_smoothness);
                }
                if key == KeyCode::KeyV {
                    smooth.position_smoothness = (smooth.position_smoothness + 0.001).abs();
                    println!("Smoothness {}", smooth.position_smoothness);
                }
            };
        }
    }
}
