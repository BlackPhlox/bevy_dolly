use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::fmt::Display;

use super::cone::Cone;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct DollyPosCtrlMoveSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct DollyPosCtrlInputSetupSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct DollyPosCtrlEntitySetupSet;

pub struct DollyPosCtrl;
impl Plugin for DollyPosCtrl {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<MoveAction>::default());
        app.init_resource::<DollyPosCtrlConfig>();
        app.add_startup_system(dolly_pos_ctrl_config_input_setup.in_set(DollyPosCtrlInputSetupSet));
        app.add_startup_system(
            dolly_pos_ctrl_config_entity_setup.in_set(DollyPosCtrlEntitySetupSet),
        );
        app.add_system(
            dolly_pos_ctrl_move_update
                .in_set(DollyPosCtrlMoveSet)
                .run_if(use_dolly_pos_ctrl_config),
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
    None,
}

#[derive(Resource)]
pub struct DollyPosCtrlConfig {
    pub enabled: bool,
    pub move_speed: f32,
    pub rot_speed: f32,
    pub pin: bool,
    pub position: Vec3,
    pub rotation: Quat,
    pub default_player: bool,
}

impl Default for DollyPosCtrlConfig {
    fn default() -> Self {
        DollyPosCtrlConfig {
            enabled: true,
            move_speed: 1.2,
            rot_speed: 0.05,
            pin: true,
            position: Vec3::new(0., 0.5, 0.),
            rotation: Quat::IDENTITY,
            default_player: true,
        }
    }
}

fn use_dolly_pos_ctrl_config(config: Res<DollyPosCtrlConfig>) -> bool {
    config.enabled
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

impl Display for DollyPosCtrlInputBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input_map = &self.input_manager.input_map;
        for (v, ma) in input_map.iter() {
            let _ = write!(f, "Action: {ma:?} -> ");
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

                let _ = write!(f, "{str}");
                if v.len() > 1 && i != v.len() - 1 {
                    let _ = write!(f, " or ");
                }

                if i == v.len() - 1 {
                    let _ = writeln!(f);
                }
            }
        }
        Ok(())
    }
}

impl Default for DollyPosCtrlInputBundle {
    fn default() -> Self {
        use MoveAction::*;
        let mut input_map = InputMap::default();
        //TODO: Impl. when added to input-manager
        //input_map.assign_gamepad(Gamepad(0));

        input_map.insert(QwertyScanCode::W, Forward);
        input_map.insert(QwertyScanCode::Up, Forward);

        input_map.insert(GamepadButtonType::DPadUp, Forward);
        //input_map.insert(SingleAxis::symmetric(GamepadAxisType::LeftStickY, 0.1), Forward); // + Y / - Y

        input_map.insert(QwertyScanCode::S, Backward);
        input_map.insert(QwertyScanCode::Down, Backward);
        input_map.insert(GamepadButtonType::DPadDown, Backward);

        input_map.insert(QwertyScanCode::A, StrafeLeft);
        input_map.insert(QwertyScanCode::Left, StrafeLeft);
        input_map.insert(GamepadButtonType::DPadLeft, StrafeLeft);

        input_map.insert(QwertyScanCode::D, StrafeRight);
        input_map.insert(QwertyScanCode::Right, StrafeRight);
        input_map.insert(GamepadButtonType::DPadRight, StrafeRight);

        //input_map.insert(SingleAxis::symmetric(GamepadAxisType::LeftStickX, 0.1), StrafeRight); // + X / - X

        input_map.insert(QwertyScanCode::Space, Up);
        input_map.insert(
            SingleAxis::positive_only(GamepadAxisType::LeftStickY, 0.1),
            Up,
        );

        input_map.insert(QwertyScanCode::LShift, Down);
        input_map.insert(
            SingleAxis::negative_only(GamepadAxisType::LeftStickY, 0.1),
            Down,
        );

        input_map.insert(QwertyScanCode::Comma, RotateLeft);
        input_map.insert(
            SingleAxis::negative_only(GamepadAxisType::LeftStickX, 0.1),
            RotateLeft,
        );

        input_map.insert(QwertyScanCode::Period, RotateRight);
        input_map.insert(
            SingleAxis::positive_only(GamepadAxisType::LeftStickX, 0.1),
            RotateRight,
        );

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

        velocity += forward * action_state.clamped_value(MoveAction::Forward);
        velocity += forward * -action_state.clamped_value(MoveAction::Backward);

        velocity += right * action_state.clamped_value(MoveAction::StrafeRight);
        velocity += right * -action_state.clamped_value(MoveAction::StrafeLeft);

        velocity += Vec3::Y * action_state.clamped_value(MoveAction::Up);
        velocity += Vec3::Y * -action_state.clamped_value(MoveAction::Down);

        if action_state.pressed(MoveAction::RotateRight) {
            //Wrapping around
            if rotation > std::f32::consts::FRAC_PI_2 * 4.0 - config.rot_speed {
                rotation = 0.0;
            }
            rotation += action_state.clamped_value(MoveAction::RotateRight) * config.rot_speed;
        } else if action_state.pressed(MoveAction::RotateLeft) {
            //Wrapping around
            if rotation < config.rot_speed {
                rotation = std::f32::consts::FRAC_PI_2 * 4.0;
            }
            let mut delta_value = action_state.clamped_value(MoveAction::RotateLeft);
            if delta_value.is_sign_positive() {
                delta_value *= -1.;
            }
            rotation += delta_value * config.rot_speed;
        }

        transform.rotation = Quat::from_rotation_y(rotation * -1.);

        //Normalize vel vector
        velocity = velocity.normalize();

        if !velocity.is_nan() {
            transform.translation += velocity * time.delta_seconds() * config.move_speed;
        }
    }
}
