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
  <div align="center">
<a href="https://crates.io/crates/bevy_dolly"><img src="https://img.shields.io/crates/v/bevy_dolly" alt="link to crates.io"></a>
<a href="https://docs.rs/bevy_dolly"><img src="https://docs.rs/bevy_dolly/badge.svg" alt="link to docs.rs"></a>
<a href="https://github.com/BlackPhlox/bevy_dolly/blob/main/LICENSE-MIT"><img src="https://img.shields.io/crates/l/bevy_dolly" alt="link to license"></a>
<a href="https://crates.io/crates/bevy_dolly"><img src="https://img.shields.io/crates/d/bevy_dolly" alt="downloads/link to crates.io"></a>
<a href="https://github.com/BlackPhlox/bevy_dolly"><img src="https://img.shields.io/github/stars/BlackPhlox/bevy_dolly" alt="stars/github repo"></a>
<a href="https://github.com/BlackPhlox/bevy_dolly/actions/workflows/main.yml"><img src="https://github.com/BlackPhlox/bevy_dolly/actions/workflows/main.yml/badge.svg" alt="github actions"></a>
<a href="https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking"><img src="https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue" alt="tracking bevy release branch"></a>
</div>
</div>
</br>

## Overview

`bevy_dolly` is a prototype plugin built for the [Bevy](https://github.com/bevyengine/bevy) game engine. It leverages the powerful `dolly` crate, developed by [h3r2tic](https://github.com/h3r2tic), to control camera movement within a Bevy application.

> [!WARNING]  
> Be aware that `bevy_dolly`'s API is still undergoing revisions. Feedback on its ergonomics and developer experience (DX) is highly appreciated.

## Dolly and Camera Movement

It's important to know that `dolly` is all about controlling how the camera moves and not changing the camera itself. This means you can use it for other things, like making a turret and its cannon move around.

Dolly operates in two essential steps:

1. **Creating a Rig**: Define a `Rig` with drivers that the dolly can utilize. These drivers, which can control both translation, rotation, constraints and custom behavior as abstractions. These rigs serve as tools to shape the camera's behavior and provide additional functionality.

2. **Marker Component**: Register a marker component on both the Camera and the Rig (rig component tag). This allows you to easily switch the behavior of a camera entity by changing the associated rig component tag. To understand the process better, refer to the [examples](#example-showcase).

## Understanding Drivers

Drivers are mechanisms that influence the behavior of the camera. They can represent constraints or provide additional functionality. To explore the available drivers, refer to the [examples](example-showcase).

In your `Cargo.toml`:

```toml
[dependencies]
# Your bevy dependency here ... 
# bevy = "0.12"
bevy_dolly = { version = "0.0.2" }
```

In your Bevy App:

```rust
// The component tag used to parent to a Dolly Rig
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

In the setup system:

```rust
// In your setup system
fn setup(
  mut commands: Commands,
) {
  commands.spawn((
    MainCamera, // The rig component tag 
    Rig::builder() // The rig itself
      .with(Position::new(Vec3::ZERO)) // Start position
      // Adds a driver with method rotate_yaw_pitch
      .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0)) 
      // Interpolation when the translation is updated, also known as smoothing
      .with(Smooth::new_position(0.3)) 
      // Interpolation when the rotation is updated (updated via the YawPitch driver)
      .with(Smooth::new_rotation(0.3)) 
      // Moves the camera point out in the Z direction and uses the position as the pivot
      .with(Arm::new(Vec3::Z * 4.0)) 
      .build(),
    Camera3dBundle::default(), // The camera which is related via the rig tag 
  ));
}
```

And your runtime to update the rig:

```rust
fn update_input(
  mut commands: Commands,
  keys: Res<Input<KeyCode>>,
  mut rig_q: Query<&mut Rig>,
) {
  let mut rig = rig_q.single_mut();
  if let Some(yaw_pitch) = rig.try_driver_mut::<YawPitch>() {
      if keys.just_pressed(KeyCode::Z) {
          yaw_pitch.rotate_yaw_pitch(-90.0, 0.0);
      }
  }
}
```

## Helper Plugins

`bevy_dolly` provides some helper plugins by default, which can be removed if not needed when setting up `bevy_dolly` as a dependency:

```toml
[dependencies]
bevy_dolly = { version = "0.0.2", default-features = false }
```

To include the drivers back, add `features = ["drivers"],` to the dependency.

## Example Showcase

Explore practical examples in the [examples repository](/examples/README.md).

### Running Examples

If you've cloned the project and want to test, execute the following command to run the `orbit` example:

```bash
cargo run --release --example orbit
```

## Compatibility and Support

|bevy|bevy_dolly|
|---|---|
|0.12|0.0.2|
|0.11|0.0.1|

## Alternatives

Explore other Bevy camera controllers that might suit your needs:

- [bevy_fps_controller](https://github.com/qhdwight/bevy_fps_controller) - A Fps controller with crouching, sprinting, flymode and more
- [smooth-bevy-cameras](https://github.com/bonsairobo/smooth-bevy-cameras) - 3 Smooth Camera controllers: Fps, Orbit or Unreal
- [bevy_spectator](https://github.com/JonahPlusPlus/bevy_spectator) - A spectator camera controller
- [bevy_flycam](https://github.com/sburris0/bevy_flycam) - A simple fly camera
- [bevy_fly_camera](https://github.com/mcpar-land/bevy_fly_camera) - A advanced fly camera
- [bevy_pancam](https://github.com/johanhelsing/bevy_pancam) - 2D Click and Drag - Style camera movement
- [bevy_config_cam](https://github.com/BlackPhlox/bevy_config_cam) - Plugin that enables to use collection of different camera controllers at runtime, uses bevy_dolly as the backend

## Licensing

The project is under a dual license: MIT and Apache 2.0. Feel free to contribute within the bounds of these licenses.

## Contributing

Yes this project is still a WIP, so PRs are very welcome.

> [!NOTE]  
> Note that the `dolly` dependency used is a slightly patched submodule to allow for native bevy transform types. To build the crate locally, run:
>
> ```bash
> git submodule update --init --recursive
> ```
