mod helpers;
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use helpers::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        // Add Dolly plugin
        .add_plugin(DollyPlugin)
        .add_event::<ChangeCamera>()
        .add_startup_system(setup)
        .add_system(change_camera_system)
        .add_system(listen_setup_camera)
        .add_startup_system(setup_example_scene)
        .run();
}

// Event for changing the Bundle Presets
#[derive(Default)]
struct ChangeCamera(ControlledType);

fn setup(mut ev_change: EventWriter<ChangeCamera>) {

    // Using the change camera system to setup
    ev_change.send(ChangeCamera(ControlledType::Free));

    info!(" Use 1, 2 to change presets");
}

fn change_camera_system(
    input_keys: Res<Input<KeyCode>>,
    mut ev_change: EventWriter<ChangeCamera>,
) {
    if input_keys.just_pressed(KeyCode::Key1) {
        ev_change.send(ChangeCamera(ControlledType::Free));
    }
    if input_keys.just_pressed(KeyCode::Key2) {
        ev_change.send(ChangeCamera(ControlledType::Fps));
    };
}

fn listen_setup_camera(
    mut commands: Commands,
    mut ev_change: EventReader<ChangeCamera>,
    mut old_entity: Local<Option<Entity>>,
) {
    for change_event in ev_change.iter() {
        // Destory current camera if it exists
        if let Some(e) = *old_entity {
            commands.entity(e).despawn()
        }

        info!("Selected {:?}", change_event.0);
        let camera = DollyControlCameraBundle {
            rig: Rig::default()
                .add(RigPosition::default())
                .add(Rotation::default())
                .add(Smooth::new(2.0, 2.0)),
            transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        };

        //let camera =  DollyControlCameraBundle::new(target);
        // Print out controls
        camera.control_actions.print_actions();
        let e = commands.spawn_bundle(camera).id();
        *old_entity = Some(e);
    }
}
