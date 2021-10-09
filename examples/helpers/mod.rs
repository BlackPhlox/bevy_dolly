use bevy::prelude::*;

#[derive(Component)]
pub struct Sheep;

/// Spawn a few basic things
pub fn setup_example_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn a sheep, gives us something to look
    spawn_sheep(Vec3::new(0.0, 0.2, 0.), &mut commands, &asset_server);

    // Spawn some ground for it to stand on
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 30.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // Create a light so we can see it once we add a camera
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

// Helper function to create a sheep
pub fn spawn_sheep(position: Vec3, commands: &mut Commands, asset_server: &AssetServer) -> Entity {
    commands
        .spawn_bundle((
            Transform::from_translation(position),
            GlobalTransform::default(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("poly_dolly.gltf#Scene0"));
        })
        .insert(Sheep)
        .id()
}

#[allow(dead_code)]
pub fn print_control_actions() {
    // TODO: Build this form the hashmap
    info!("Using 'DollyControlCameraBundle', these are the default binding");
    info!("Forward: {:?}", vec![KeyCode::Up, KeyCode::W]);
    info!("Backward: {:?}", vec![KeyCode::Down, KeyCode::S]);
    info!("Left: {:?}", vec![KeyCode::Left, KeyCode::A]);
    info!("Right: {:?}", vec![KeyCode::Right, KeyCode::D]);
    info!("Up: {:?}", vec![KeyCode::Z]);
    info!("Down: {:?}", vec![KeyCode::X]);
    info!("RotateLeft: {:?}", vec![KeyCode::Q]);
    info!("RotateRight: {:?}", vec![KeyCode::E]);
    info!("Boost: {:?}", vec![KeyCode::LShift]);
}

// TODO: add cursor stuff back once everything works well
/// Grabs/ungrabs mouse cursor
#[allow(dead_code)]
pub fn toggle_grab_cursor(window: &mut Window) {
    
    if window.cursor_visible() {
        info!("here");
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
    }
}

/// Grabs the cursor when game first starts
#[allow(dead_code)]
pub fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    toggle_grab_cursor(windows.get_primary_mut().unwrap());
}

#[allow(dead_code)]
pub fn cursor_grab_system(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(window);
    }
}
