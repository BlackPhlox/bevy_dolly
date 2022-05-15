use bevy::{
    core::Time,
    ecs::schedule::ShouldRun,
    math::{Quat, Vec3},
    pbr::PbrBundle,
    prelude::{
        App, Assets, BuildChildren, Bundle, Color, Commands, Component, GamepadButtonType,
        GlobalTransform, KeyCode, Mesh, Plugin, Query, Res, ResMut, StandardMaterial, SystemSet,
        Transform, With,
    },
};
use leafwing_input_manager::prelude::*;

use super::cone::Cone;

pub struct DollyPosCtrl;
impl Plugin for DollyPosCtrl {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<MoveAction>::default());
        app.init_resource::<DollyPosCtrlConfig>();
        app.add_startup_system(dolly_pos_ctrl_config_input_setup);
        app.add_startup_system(dolly_pos_ctrl_config_entity_setup);
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(use_dolly_pos_ctrl_config)
                .with_system(dolly_pos_ctrl_move_update),
        );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum MoveAction {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    Up,
    Down,
    RotateLeft,
    RotateRight,
}

struct DollyPosCtrlConfig {
    enabled: bool,
    speed: f32,
    position: Vec3,
}

impl Default for DollyPosCtrlConfig {
    fn default() -> Self {
        DollyPosCtrlConfig {
            enabled: true,
            speed: 1.2,
            position: bevy::math::Vec3::new(0., 0.5, 0.),
        }
    }
}

fn use_dolly_pos_ctrl_config(config: Res<DollyPosCtrlConfig>) -> ShouldRun {
    if config.enabled {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

#[derive(Component)]
struct DollyPosCtrlAction;

fn dolly_pos_ctrl_config_input_setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(DollyPosCtrlAction)
        .insert_bundle(DollyPosCtrlInputBundle::default());
}

#[derive(Component)]
pub struct DollyPosCtrlMove;

#[derive(Bundle)]
struct DollyPosCtrlInputBundle {
    #[bundle]
    input_manager: InputManagerBundle<MoveAction>,
}

impl Default for DollyPosCtrlInputBundle {
    fn default() -> Self {
        use MoveAction::*;
        let mut input_map = InputMap::default();
        //TODO: Impl. when added to input-manager
        //input_map.assign_gamepad(Gamepad(0));

        input_map.insert(Forward, KeyCode::W);
        input_map.insert(Forward, KeyCode::Up);
        //input_map.insert(Forward, GamepadAxisType::LeftStickY); +Y

        input_map.insert(Backward, KeyCode::S);
        input_map.insert(Backward, KeyCode::Down);
        //input_map.insert(Forward, GamepadAxisType::LeftStickY); -Y

        input_map.insert(StrafeLeft, KeyCode::A);
        input_map.insert(StrafeLeft, KeyCode::Left);
        //input_map.insert(StrafeLeft, GamepadAxisType::LeftStickX); +X

        input_map.insert(StrafeRight, KeyCode::D);
        input_map.insert(StrafeRight, KeyCode::Right);
        //input_map.insert(StrafeLeft, GamepadAxisType::LeftStickX); -X

        input_map.insert(Up, KeyCode::Space);
        input_map.insert(Up, GamepadButtonType::DPadUp);

        input_map.insert(Down, KeyCode::LShift);
        input_map.insert(Down, GamepadButtonType::DPadDown);

        input_map.insert(RotateLeft, KeyCode::Comma);

        input_map.insert(RotateRight, KeyCode::Period);

        for (v, _) in input_map.iter() {
            print!("Action: {:?} -> ", v);
            if let a = input_map.get(v) {
                for (i, b) in a.iter().enumerate() {
                    let str = match b {
                        UserInput::Single(x) => {
                            format!("Press {}", &x)
                        }
                        UserInput::Chord(x) => {
                            format!("Press and hold {:?}", &x)
                        }
                    };
                    print!("{}", str);
                    if a.len() > 1 && i != a.len() - 1 {
                        print!(" or ");
                    }

                    if i == a.len() - 1 {
                        println!();
                    }
                }
            }
        }

        let input_manager = InputManagerBundle {
            input_map,
            action_state: ActionState::default(),
        };

        Self { input_manager }
    }
}

fn dolly_pos_ctrl_config_entity_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<DollyPosCtrlConfig>,
) {
    let cone_mesh = meshes.add(Mesh::from(Cone {
        height: 0.2,
        radius: 0.1,
        subdivisions: 5,
    }));

    let player_mat = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 0.0, 0.0, 0.5),
        unlit: true,
        ..Default::default()
    });

    commands
        .spawn_bundle((
            Transform {
                rotation: Quat::IDENTITY,
                translation: config.position,
                ..Default::default()
            },
            GlobalTransform::identity(),
        ))
        .with_children(|cell| {
            cell.spawn_bundle(PbrBundle {
                mesh: cone_mesh.clone(),
                material: player_mat.clone(),
                transform: Transform::from_rotation(Quat::from_rotation_x(
                    std::f32::consts::FRAC_PI_2,
                )),
                ..Default::default()
            });
        })
        .insert(DollyPosCtrlMove);
}

fn dolly_pos_ctrl_move_update(
    time: Res<Time>,
    config: Res<DollyPosCtrlConfig>,
    mut transforms: Query<(&DollyPosCtrlMove, &mut Transform)>,
    act_query: Query<&ActionState<MoveAction>, With<DollyPosCtrlAction>>,
) {
    let action_state = act_query.single();

    for (_player, mut transform) in transforms.iter_mut() {
        let (_, mut rotation) = transform.rotation.to_axis_angle();
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = Vec3::new(local_z.x, 0., local_z.z);
        let right = transform.rotation * -Vec3::X;

        if action_state.pressed(MoveAction::Forward) {
            velocity += forward
        }
        if action_state.pressed(MoveAction::Backward) {
            velocity -= forward
        }
        if action_state.pressed(MoveAction::Up) {
            velocity += Vec3::Y
        }
        if action_state.pressed(MoveAction::Down) {
            velocity -= Vec3::Y
        }
        if action_state.pressed(MoveAction::StrafeLeft) {
            velocity -= right
        }
        if action_state.pressed(MoveAction::StrafeRight) {
            velocity += right
        }
        if action_state.pressed(MoveAction::RotateLeft) {
            //Wrapping around
            if rotation > std::f32::consts::FRAC_PI_2 * 4.0 - 0.05 {
                rotation = 0.0;
            }
            rotation += 0.1
        }
        if action_state.pressed(MoveAction::RotateRight) {
            //Wrapping around
            if rotation < 0.05 {
                rotation = std::f32::consts::FRAC_PI_2 * 4.0;
            }
            rotation -= 0.1
        }

        velocity = velocity.normalize();

        transform.rotation = Quat::from_rotation_y(rotation);

        if !velocity.is_nan() {
            transform.translation += velocity * time.delta_seconds() * config.speed;
        }
    }
}
