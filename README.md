<h1>bevy_dolly</h1>
<div align="center">
<table>
  <tr>
    <th>Static</th>
    <th>Pinned</th>
  </tr>
  <tr>
    <td><a href="https://github.com/BlackPhlox/bevy_dolly"><img src="https://raw.githubusercontent.com/BlackPhlox/BlackPhlox/master/bevy_dolly_1.svg" alt="bevy dolly static"></a></td>
    <td><a href="https://github.com/BlackPhlox/bevy_dolly"><img src="https://raw.githubusercontent.com/BlackPhlox/BlackPhlox/master/bevy_dolly_dev_0.svg" alt="bevy dolly pinned"></a></td>
  </tr>
</table>
</div>

`bevy_dolly` is a prototype plugin using [h3r2tic](https://github.com/h3r2tic)'s powerful crate: [dolly](https://github.com/h3r2tic/dolly), implemented for bevy.<br/>

It is important to note that dolly is a way to control the movement of the camera and thus, not the camera component itself. </br>

Dolly requires two steps to function:
1. Creating a `Rig` we are able to define drivers on which the dolly can enact, these drivers can both be constraints and functionality.
2. A marker component that is registered on both the Camera and the Rig.

## What are drivers?

Explain what drivers are

To read more about the different drivers.

```rs
#[derive(Component)]
struct MainCamera;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    //..
    .add_system(Dolly::<MainCamera>::update_active)
    //..
    .run();
}
```

```rs
// In your setup system
fn setup(
  mut commands: Commands,
) {
  commands.spawn((
    MainCamera, // The rig tag
    Rig::builder()
      .with(Position::new(Vec3::ZERO))
      .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
      .with(Smooth::new_position(0.3))
      .with(Smooth::new_rotation(0.3))
      .with(Arm::new(Vec3::Z * 4.0))
      .build(),
    Camera3dBundle::default(),
  ));
}
```
Link to [examples readme](/examples/README.md)

## Examples

To see how works in bevy in practice, please look at this repository's [examples](/examples/).

Something about usages...

Reference 

### How to run

`cargo run --release --example orbit`

## Support
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

|bevy|bevy_dolly|
|---|---|
|0.10| 0.0.1 |

## Alternatives

There is a bunch of other bevy camera controllers that are worth checking out, especially if you are just starting out learning bevy:

- [smooth-bevy-cameras](https://github.com/bonsairobo/smooth-bevy-cameras) - 3 Smooth Camera controllers: Fps, Orbit or Unreal
- [bevy_spectator](https://github.com/JonahPlusPlus/bevy_spectator) - A spectator camera controller
- [bevy_flycam](https://github.com/sburris0/bevy_flycam) - A simple fly camera
- [bevy_fly_camera](https://github.com/mcpar-land/bevy_fly_camera)  - A advanced fly camera
- [bevy_config_cam](https://github.com/BlackPhlox/bevy_config_cam) - Plugin that enables to use collection of different camera controllers at runtime, uses bevy_dolly as the backend

## Licensing
The project is under dual license MIT and Apache 2.0, so joink to your hearts content, just remember the license agreements.

## Contributing
Yes this project is still very much WIP, so PRs are very welcome
