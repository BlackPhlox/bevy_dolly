use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::cone::Cone;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct DollyPosCtrlMoveSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct DollyPosCtrlInputSetupSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct DollyPosCtrlEntitySetupSet;

/// This plugin is a simple player controller
/// Add the Bevy Component: DollyPosCtrlMove
/// to an entity to allow that entity's transform to
/// be mutated by the plugin. Use code below in the
/// app building step to override default behavior of the plugin:
/// ```rs
/// use bevy::prelude::*;
/// use bevy_dolly::prelude::*;
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugin(DollyPosCtrl)
///         .insert_resource(DollyPosCtrlConfig {
///             ..Default::default()
///     }).run();
/// }
///
/// fn setup(mut commands: Commands){
///     commands.spawn(/* Your player entity here */, DollyPosCtrlMove));
/// }
/// ```
pub struct DollyPosCtrl;
impl Plugin for DollyPosCtrl {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MoveAction>::default());
        app.init_resource::<DollyPosCtrlConfig>();
        app.add_systems(
            Startup,
            (
                dolly_pos_ctrl_config_input_setup.in_set(DollyPosCtrlInputSetupSet),
                dolly_pos_ctrl_config_entity_setup.in_set(DollyPosCtrlEntitySetupSet),
            ),
        );
        app.add_systems(
            Update,
            dolly_pos_ctrl_move_update
                .in_set(DollyPosCtrlMoveSet)
                .run_if(use_dolly_pos_ctrl_config),
        );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
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
    pub transform: Transform,
    pub player: DollyCameraPlayer,
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum DollyCameraPlayer {
    #[default]
    DefaultPlayer,
    Entity(Entity),
    None,
}

impl Default for DollyPosCtrlConfig {
    fn default() -> Self {
        DollyPosCtrlConfig {
            enabled: true,
            move_speed: 1.2,
            rot_speed: 0.05,
            pin: true,
            transform: Transform::from_translation(Vec3::new(0., 0.5, 0.))
                .with_rotation(Quat::IDENTITY),
            player: DollyCameraPlayer::default(),
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
    input_manager: InputManagerBundle<MoveAction>,
}

/* impl Display for DollyPosCtrlInputBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input_map = &self.input_manager.input_map;
        for (ma, v) in input_map.iter_buttonlike() {
            let _ = write!(f, "Action: {ma:?} -> ");
            for (i, b.) in v.iter().enumerate() {
                let str = match b.value(input_store, gamepad) {
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
} */

impl Default for DollyPosCtrlInputBundle {
    fn default() -> Self {
        use MoveAction::*;
        let input_map = InputMap::default() 
        //TODO: Impl. when added to input-manager
        //input_map.assign_gamepad(Gamepad(0));

        .with(Forward, KeyCode::KeyW)
        .with(Forward, KeyCode::ArrowUp)

        .with(Forward, GamepadButton::DPadUp)
        //.with(Forward, SingleAxis::symmetric(GamepadAxisType::LeftStickY, 0.1)) // + Y / - Y

        .with(Backward, KeyCode::KeyS)
        .with(Backward, KeyCode::ArrowDown)
        .with(Backward, GamepadButton::DPadDown)

        .with(StrafeLeft, KeyCode::KeyA)
        .with(StrafeLeft, KeyCode::ArrowLeft)
        .with(StrafeLeft, GamepadButton::DPadLeft)

        .with(StrafeRight, KeyCode::KeyD)
        .with(StrafeRight, KeyCode::ArrowRight)
        .with(StrafeRight, GamepadButton::DPadRight)

        //.with(StrafeRight, SingleAxis::symmetric(GamepadAxisType::LeftStickX, 0.1)) // + X / - X

        /* 
        .with(Up, KeyCode::Space)
        .with_axis(
            Up,
           
              GamepadControlAxis::LEFT_Y.only_positive(0.1)
        )

        .with(Down, KeyCode::ShiftLeft)
        .with_axis(
            Down,
          
              GamepadControlAxis::LEFT_Y.only_negative(0.1)
        )
        .with(RotateLeft, KeyCode::Comma)
        .with_axis(
            RotateLeft, 
            GamepadControlAxis::LEFT_X.only_negative(0.1)
        )
        .with(RotateRight, KeyCode::Period)
        .with_axis(
            RotateRight,
            GamepadControlAxis::LEFT_X.only_positive(0.1)
        );
        */;

        let input_manager = InputManagerBundle {
            input_map,
            action_state: ActionState::default(),
        };

        Self { input_manager }
    }
}

fn spawn_default_player(
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
        base_color: Color::srgba(1.0, 0.0, 0.0, 0.5),
        unlit: true,
        ..default()
    });

    commands
        .spawn(config.transform)
        .with_children(|cell| {
            cell.spawn(
                (
                    Mesh3d(  cone_mesh.clone() ),
                    MeshMaterial3d( player_mat.clone() ),
                    Transform::from_rotation(Quat::from_rotation_x(
                       std::f32::consts::FRAC_PI_2,
                   )),
                )
            );
        })
        .insert(DollyPosCtrlMove);
}

#[allow(unused_mut)]
fn dolly_pos_ctrl_config_entity_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<DollyPosCtrlConfig>,
) {
    match config.player {
        DollyCameraPlayer::DefaultPlayer => {
            spawn_default_player(commands, meshes, materials, config);
        }
        DollyCameraPlayer::Entity(e) => {
            commands.entity(e).insert(DollyPosCtrlMove);
        }
        DollyCameraPlayer::None => (),
    }
}

fn dolly_pos_ctrl_move_update(
    time: Res<Time>,
    config: Res<DollyPosCtrlConfig>,
    mut transforms: Query<&mut Transform, With<DollyPosCtrlMove>>,
    act_query: Query<&ActionState<MoveAction>, With<DollyPosCtrlAction>>,
) {
    let action_state = act_query.single();

    for mut transform in transforms.iter_mut() {
        let (_, mut rotation) = transform.rotation.to_axis_angle();
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = Vec3::new(local_z.x, 0., local_z.z);
        let right = transform.rotation * -Vec3::X;

        velocity += forward * action_state.clamped_value(&MoveAction::Forward);
        velocity += forward * -action_state.clamped_value(&MoveAction::Backward);

        velocity += right * action_state.clamped_value(&MoveAction::StrafeRight);
        velocity += right * -action_state.clamped_value(&MoveAction::StrafeLeft);

        velocity += Vec3::Y * action_state.clamped_value(&MoveAction::Up);
        velocity += Vec3::Y * -action_state.clamped_value(&MoveAction::Down);

        if action_state.pressed(&MoveAction::RotateRight) {
            //Wrapping around
            if rotation > std::f32::consts::FRAC_PI_2 * 4.0 - config.rot_speed {
                rotation = 0.0;
            }
            rotation += action_state.clamped_value(&MoveAction::RotateRight) * config.rot_speed;
        } else if action_state.pressed(&MoveAction::RotateLeft) {
            //Wrapping around
            if rotation < config.rot_speed {
                rotation = std::f32::consts::FRAC_PI_2 * 4.0;
            }
            let mut delta_value = action_state.clamped_value(&MoveAction::RotateLeft);
            if delta_value.is_sign_positive() {
                delta_value *= -1.;
            }
            rotation += delta_value * config.rot_speed;
        }

        transform.rotation = Quat::from_rotation_y(rotation * -1.);

        //Normalize vel vector
        velocity = velocity.normalize();

        if !velocity.is_nan() {
            transform.translation += velocity * time.delta_secs() * config.move_speed;
        }
    }
}
