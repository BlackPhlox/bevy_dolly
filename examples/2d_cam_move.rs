use bevy::prelude::*;
use bevy_dolly::prelude::*;

#[derive(Component)]
struct MainCamera;

const SPEED: f32 = 4.5;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        //If large amount of smoothing is used, where camera movement is expected beyond the time of input
        //Ie. motion smoothing beyond 0.25, use update_2d_active_continuous instead
        .add_systems((Dolly::<MainCamera>::update_2d_active, update_camera))
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("bevy_dolly.png"),
        transform: Transform::from_xyz(100., 0., 0.),
        sprite: Sprite {
            custom_size: Some(Vec2::new(128., 128.)),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
        Rig::builder()
            .with(Position::default())
            .with(Smooth::new_position(0.25))
            .build(),
    ));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("room.png"),
        transform: Transform::from_xyz(100., 0., 0.),
        sprite: Sprite {
            custom_size: Some(Vec2::new(2.6 * 800., 800.)),
            ..Default::default()
        },
        ..Default::default()
    });

    info!("Use W, A, S, D for movement");
    info!("Use Z & X zooming in and out");
}

fn update_camera(keys: Res<Input<KeyCode>>, mut query: Query<&mut Rig>) {
    for mut rig in &mut query {
        for &key in keys.get_pressed() {
            let pos_driver = rig.try_driver_mut::<Position>();
            if let Some(pos) = pos_driver {
                if key == KeyCode::W {
                    pos.translate(SPEED * Vec3::Y);
                }
                if key == KeyCode::A {
                    pos.translate(SPEED * -Vec3::X);
                }
                if key == KeyCode::S {
                    pos.translate(SPEED * -Vec3::Y);
                }
                if key == KeyCode::D {
                    pos.translate(SPEED * Vec3::X);
                }
                if key == KeyCode::Z {
                    pos.translate(SPEED * -Vec3::Z);
                }
                if key == KeyCode::X {
                    pos.translate(SPEED * Vec3::Z);
                }
            }

            let smooth_driver = rig.try_driver_mut::<Smooth>();
            if let Some(smooth) = smooth_driver {
                if key == KeyCode::C {
                    smooth.position_smoothness = (smooth.position_smoothness - 0.001).abs();
                    println!("Smoothness {}", smooth.position_smoothness);
                }
                if key == KeyCode::V {
                    smooth.position_smoothness = (smooth.position_smoothness + 0.001).abs();
                    println!("Smoothness {}", smooth.position_smoothness);
                }
            };
        }
    }
}
