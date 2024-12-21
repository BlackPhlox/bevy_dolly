use bevy::prelude::*;
use bevy_dolly::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                Dolly::<MainCamera>::update_2d_active,
                update_camera,
                sprite_movement,
            ),
        )
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
    let offset_transform = Transform::from_xyz(100., 0., 0.);

    let mut dolly = Sprite::from_image(asset_server.load("bevy_dolly.png"));
    dolly.custom_size = Some(Vec2::new(128., 128.));
    commands.spawn((
        dolly,
        offset_transform.with_translation(Vec3 {
            x: 0.,
            y: 0.,
            z: 1.,
        }),
        Direction::Right,
    ));

    let mut room = Sprite::from_image(asset_server.load("room.png"));
    room.custom_size = Some(Vec2::new(2.6 * 800., 800.));
    commands.spawn((
        room,
        offset_transform.with_translation(Vec3 {
            x: 0.,
            y: 200.,
            z: 0.,
        }),
    ));

    let mut room_end = Sprite::from_image(asset_server.load("room_end.png"));
    commands.spawn((room_end.clone(), Transform::from_xyz(1016., -104.5, 2.0)));

    room_end.flip_x = true;
    commands.spawn((room_end, Transform::from_xyz(-1016., -104.5, 2.0)));

    commands.spawn((
        MainCamera,
        Rig::builder()
            .with(Position::new(Vec3::new(0., 0., 0.)))
            .with(Smooth::new_position(1.2))
            .build(),
        Camera2d,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Right => transform.translation.x += 400. * time.delta_secs(),
            Direction::Left => transform.translation.x -= 400. * time.delta_secs(),
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
