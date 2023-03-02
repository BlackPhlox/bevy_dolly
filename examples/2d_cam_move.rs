use bevy::prelude::*;
use bevy_dolly::prelude::*;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_dolly_2d_component(MainCamera)
        .add_system(update_camera)
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
        MainCamera,
        Rig::builder()
            .with(Position::new(Vec3::new(0., 0., 400.)))
            .with(Smooth::new_position(1.2))
            .build(),
    ));

    commands.spawn(Camera2dBundle::default()).insert(MainCamera);

    commands.spawn(SpriteBundle {
        texture: asset_server.load("room.png"),
        transform: Transform::from_xyz(100., 0., 0.),
        sprite: Sprite {
            custom_size: Some(Vec2::new(2.6 * 800., 800.)),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn update_camera(keys: Res<Input<KeyCode>>, mut query: Query<&mut Rig>) {
    let mut rig = query.single_mut();
    let camera_driver = rig.driver_mut::<Position>();
    let speed = 4.5;

    for &key in keys.get_pressed() {
        if key == KeyCode::W {
            camera_driver.translate(speed * Vec3::new(0., 1., 0.));
        }
        if key == KeyCode::A {
            camera_driver.translate(speed * Vec3::new(-1., 0., 0.));
        }
        if key == KeyCode::S {
            camera_driver.translate(speed * Vec3::new(0., -1., 0.));
        }
        if key == KeyCode::D {
            camera_driver.translate(speed * Vec3::new(1., 0., 0.));
        }
        if key == KeyCode::Z {
            camera_driver.translate(speed * Vec3::new(0., 0., -1.));
        }
        if key == KeyCode::X {
            camera_driver.translate(speed * Vec3::new(0., 0., 1.));
        }
    }
}
