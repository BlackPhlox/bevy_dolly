use bevy::prelude::*;
use bevy_dolly::{prelude::*, system::DollyComponent};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_dolly_component(MainCamera)
        .add_system(sprite_movement)
        .add_system(update_camera)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("bevy_dolly.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(128., 128.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Direction::Right);

    commands.spawn(SpriteBundle {
        texture: asset_server.load("room.png"),
        transform: Transform::from_xyz(100., 200., 0.),
        sprite: Sprite {
            custom_size: Some(Vec2::new(2.6 * 800., 800.)),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("room_end.png"),
        transform: Transform::from_xyz(1116., -104.5, 0.),
        sprite: Sprite {
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("room_end.png"),
        transform: Transform::from_xyz(-916., -104.5, 0.),
        sprite: Sprite {
            flip_x: true,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        MainCamera,
        Rig::builder()
            .with(Position::new(Vec3::new(0., 0., 0.)))
            .with(Smooth::new_position(1.2))
            .build(),
    ));

    commands.spawn((MainCamera, Camera2dBundle::default()));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Right => transform.translation.x += 400. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 400. * time.delta_seconds(),
        }

        if transform.translation.x > 1200. {
            *logo = Direction::Left;
        } else if transform.translation.x < -1200. {
            *logo = Direction::Right;
        }
    }
}

fn update_camera(sprite_position: Query<(&Direction, &Transform)>, mut q0: Query<&mut Rig>) {
    let mut rig = q0.single_mut();
    let camera_driver = rig.driver_mut::<Position>();

    for (_dir, pos) in &sprite_position {
        if pos.translation.x < 495. && pos.translation.x > -295. {
            let dolly_transform = DollyTransform::from(pos);
            camera_driver.position = dolly_transform.position;
        }
    }
}
