# Examples

To run an example:

- Clone the repository using `git clone --recurse-submodules https://github.com/BlackPhlox/bevy_dolly`
- Run `cargo r --release --example x` where `x` is the name of one of the examples below: 

## `2d_cam_move`

2D example of how to setup a simple movement controller. 
Comparable to FlyCamera2d mode of [bevy_fly_camera](https://github.com/mcpar-land/bevy_fly_camera/blob/master/README.md#2d).

## `2d_edge_snap`

2D example of creating a level with a camera controller comparable to a 2D street brawler.

## `custom`

A custom driver implementation using nested existing drivers and its registration in bevy to get an understanding on how users can create nested drivers themselves.

## `follow`

Simple camera example following a player.

(TODO): May be removed in place of `custom.rs`

## `fpv`

Example showing the Fpv driver. Camera controller is comparable to :

- [bevy_fly_camera](https://github.com/mcpar-land/bevy_fly_camera) in 3D mode.

- [bevy_flycam](https://github.com/sburris0/bevy_flycam)

## `look_at`

Simple default camera example of tracking (the `LookAt` driver) the player (Cone).

## `orbit`

An extensive example showing orbit capabilities of the library.

## `split`

Example showing using multiple cameras and drivers at the same time.

## `switch`

Simple example showing how to create a controller that allows to switch between players/moving targets.