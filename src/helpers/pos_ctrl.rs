use bevy::{
    ecs::schedule::ShouldRun,
    math::{Quat, Vec3},
    pbr::PbrBundle,
    prelude::{
        default, App, Assets, BuildChildren, Bundle, Color, Commands, Component, GamepadButtonType,
        IntoSystemDescriptor, KeyCode, Mesh, Plugin, Query, Res, ResMut, Resource, SpatialBundle,
        StandardMaterial, SystemLabel, SystemSet, Time, Transform, With,
    },
};
use leafwing_input_manager::prelude::*;

use super::cone::Cone;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub struct DollyPosCtrlMoveLabel;

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
                .with_system(dolly_pos_ctrl_move_update.label(DollyPosCtrlMoveLabel)),
        );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum MoveAction {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    Up,
    Down,
    RotateLeft,
    RotateRight,
}

#[derive(Resource)]
pub struct DollyPosCtrlConfig {
    pub enabled: bool,
    pub speed: f32,
    pub position: Vec3,
    pub rotation: Quat,
    pub default_player: bool,
}

impl Default for DollyPosCtrlConfig {
    fn default() -> Self {
        DollyPosCtrlConfig {
            enabled: true,
            speed: 1.2,
            position: bevy::math::Vec3::new(0., 0.5, 0.),
            rotation: bevy::math::Quat::IDENTITY,
            default_player: true,
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
    commands.spawn((DollyPosCtrlAction, DollyPosCtrlInputBundle::default()));
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

        input_map.insert(KeyCode::W, Forward);
        input_map.insert(KeyCode::Up, Forward);
        //input_map.insert(Forward, GamepadAxisType::LeftStickY); +Y

        input_map.insert(KeyCode::S, Backward);
        input_map.insert(KeyCode::Down, Backward);
        //input_map.insert(Forward, GamepadAxisType::LeftStickY); -Y

        input_map.insert(KeyCode::A, StrafeLeft);
        input_map.insert(KeyCode::Left, StrafeLeft);
        //input_map.insert(StrafeLeft, GamepadAxisType::LeftStickX); +X

        input_map.insert(KeyCode::D, StrafeRight);
        input_map.insert(KeyCode::Right, StrafeRight);
        //input_map.insert(StrafeLeft, GamepadAxisType::LeftStickX); -X

        input_map.insert(KeyCode::Space, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::LShift, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::Comma, RotateLeft);

        input_map.insert(KeyCode::Period, RotateRight);

        for (v, ma) in input_map.iter() {
            print!("Action: {ma:?} -> ");
            for (i, b) in v.iter().enumerate() {
                let str = match b {
                    UserInput::Single(x) => {
                        format!("Press {}", &x)
                    }
                    UserInput::Chord(x) => {
                        let k = x
                            .iter()
                            .map(|f| f.to_string())
                            .collect::<Vec<String>>()
                            .join(" + ");

                        format!("Press and hold {}", &k)
                    }
                    x => format!("Unknown input {:?}", &x),
                };

                print!("{str}");
                if v.len() > 1 && i != v.len() - 1 {
                    print!(" or ");
                }

                if i == v.len() - 1 {
                    println!();
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
    if !config.default_player {
        return;
    }

    let cone_mesh = meshes.add(Mesh::from(Cone {
        height: 0.2,
        radius: 0.1,
        subdivisions: 5,
    }));

    let player_mat = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 0.0, 0.0, 0.5),
        unlit: true,
        ..default()
    });

    commands
        .spawn(SpatialBundle::from_transform(Transform {
            rotation: Quat::IDENTITY,
            translation: config.position,
            ..default()
        }))
        .with_children(|cell| {
            cell.spawn(PbrBundle {
                mesh: cone_mesh.clone(),
                material: player_mat.clone(),
                transform: Transform::from_rotation(Quat::from_rotation_x(
                    std::f32::consts::FRAC_PI_2,
                )),
                ..default()
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
            velocity += forward;
        }
        if action_state.pressed(MoveAction::Backward) {
            velocity -= forward;
        }
        if action_state.pressed(MoveAction::Up) {
            velocity += Vec3::Y;
        }
        if action_state.pressed(MoveAction::Down) {
            velocity -= Vec3::Y;
        }
        if action_state.pressed(MoveAction::StrafeLeft) {
            velocity -= right;
        }
        if action_state.pressed(MoveAction::StrafeRight) {
            velocity += right;
        }
        if action_state.pressed(MoveAction::RotateLeft) {
            //Wrapping around
            if rotation > std::f32::consts::FRAC_PI_2 * 4.0 - 0.05 {
                rotation = 0.0;
            }
            rotation += 0.05;
        }
        if action_state.pressed(MoveAction::RotateRight) {
            //Wrapping around
            if rotation < 0.05 {
                rotation = std::f32::consts::FRAC_PI_2 * 4.0;
            }
            rotation -= 0.05;
        }

        velocity = velocity.normalize();

        transform.rotation = Quat::from_rotation_y(rotation);

        if !velocity.is_nan() {
            transform.translation += velocity * time.delta_seconds() * config.speed;
        }
    }
}
