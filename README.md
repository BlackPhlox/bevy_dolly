<h1>bevy_dolly</h1>
<div align="left">
<a href="https://github.com/BlackPhlox/bevy_dolly"><img src="https://raw.githubusercontent.com/BlackPhlox/BlackPhlox/master/bevy_dolly_1.svg" alt="bevy_dolly"></a>
</div><br/>

`bevy_dolly` is [h3r2tic](https://github.com/h3r2tic)'s powerful crate: [dolly](https://github.com/h3r2tic/dolly), implemented in bevy.<br/>

It is important to note that dolly is a way to translate the camera and thus, not the camera component itself. </br>

Dolly requires two steps to function:
1. Setup - Using `CameraRig` we are able to define drivers on which the dolly can enact, these drivers can both be constraints and functionality.
2. Update - Querying for `CameraRig` allows us to mutate its drivers and update the cameras translation to reflect the mutated changes.

To see how this is done in bevy, please look at this repository's [examples](/examples/).

As this plugin is still in its prototype phase. Currently, the plugin only contains helper methods for converting between bevy and dolly's glam crate. A more standalone plugin will come later.

_Please do not call `dolly.clone()`, she have already been cloned once._

## How to run

`cargo run --release --example orbit`

## Support
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

|bevy|bevy_dolly|
|---|---|
|0.5| 0.0.X |
## Licensing
The project is under dual license MIT and Apache 2.0, so joink to your hearts content, just remember the license agreements.

## Contributing
Yes this project is still very much WIP, so PRs are very welcome
