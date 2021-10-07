use bevy::prelude::*;

/// Spawn a few basic things
pub fn spawn_example_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    // Spawn a sheep, gives us something to look
    commands
        .spawn_bundle((
            Transform::from_xyz(0., 0.2, 0.),
            GlobalTransform::default(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("poly_dolly.gltf#Scene0"));
        })
        .id();

    // Spawn some ground for it to stand on
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // Create a light so we can see it once we add a camera
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

