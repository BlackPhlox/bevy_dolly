<h1>bevy_dolly</h1>
</div>
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

`bevy_dolly` is a prototype plugin using [h3r2tic](https://github.com/h3r2tic)'s powerful crate: [dolly](https://github.com/h3r2tic/dolly), implemented for bevy.<br/>

It is important to note that dolly is a way to translate the camera and thus, not the camera component itself. </br>

Dolly requires two steps to function:
1. Creating a `CameraRig` we are able to define drivers on which the dolly can enact, these drivers can both be constraints and functionality.
2. A marker component that is inserted on both the Camera and the Rig

To see how this is done in bevy, please look at this repository's [examples](/examples/).

## How to run

`cargo run --release --example orbit`

## Support
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

|bevy|bevy_dolly|
|---|---|
|0.8| 0.0.X |
## Licensing
The project is under dual license MIT and Apache 2.0, so joink to your hearts content, just remember the license agreements.

## Contributing
Yes this project is still very much WIP, so PRs are very welcome
