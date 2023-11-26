use bevy::prelude::*;
use bevy_dolly::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, sprite_movement)
        .add_systems(Update, update_camera)
        .add_systems(Update, draw_gizmo)
        .add_systems(Update, Dolly::<MainCamera>::update_2d_active_continuous)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct PlayerOne;

#[derive(Component)]
struct PlayerTwo;

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("bevy_dolly.png"),
            transform: Transform::from_xyz(100., 0., 0.1),
            sprite: Sprite {
                custom_size: Some(Vec2::new(128., 128.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Direction::Right)
        .insert(PlayerOne);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("bevy_dolly.png"),
            transform: Transform::from_xyz(100., 0., 0.1),
            sprite: Sprite {
                custom_size: Some(Vec2::new(128., 128.)),
                flip_x: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PlayerTwo);

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
        transform: Transform::from_xyz(1116., -104.5, 0.2),
        sprite: Sprite {
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("room_end.png"),
        transform: Transform::from_xyz(-916., -104.5, 0.2),
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
        Camera2dBundle::default(),
    ));
}

fn draw_gizmo(mut gizmos: Gizmos, q0: Query<&Transform, With<MainCamera>>) {
    for a in &q0 {
        gizmos.rect_2d(
            a.translation.truncate(),
            0.,
            Vec2::splat(300.),
            Color::BLACK,
        );
    }
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
            camera_driver.position = pos.translation;
        }
    }
}
