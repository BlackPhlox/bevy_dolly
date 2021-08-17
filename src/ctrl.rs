use bevy::{
    core::Time,
    ecs::schedule::ShouldRun,
    input::Input,
    math::{Quat, Vec3},
    pbr::PbrBundle,
    prelude::{
        AppBuilder, Assets, BuildChildren, Color, Commands, Entity, GlobalTransform, IntoSystem,
        KeyCode, Mesh, Plugin, Query, Res, ResMut, StandardMaterial, SystemSet, Transform,
    },
};

use crate::cone::Cone;

pub struct Ctrl;
impl Plugin for Ctrl {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CtrlConfig>()
            .add_startup_system(ctrl_setup.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(use_ctrl.system())
                    .with_system(ctrl_update.system()),
            );
    }
}

pub struct WASDKeyMap {
    pub forward: &'static [KeyCode],
    pub backward: &'static [KeyCode],
    pub left: &'static [KeyCode],
    pub right: &'static [KeyCode],
    pub up: &'static [KeyCode],
    pub down: &'static [KeyCode],
    pub rot_left: &'static [KeyCode],
    pub rot_right: &'static [KeyCode],
}

impl Default for WASDKeyMap {
    fn default() -> Self {
        Self {
            forward: &[KeyCode::Up],
            backward: &[KeyCode::Down],
            left: &[KeyCode::Comma],
            right: &[KeyCode::Period],
            up: &[KeyCode::RShift],
            down: &[KeyCode::Minus],
            rot_left: &[KeyCode::Left],
            rot_right: &[KeyCode::Right],
        }
    }
}

pub struct CtrlConfig {
    pub enabled: bool,
    pub position: Vec3,
    pub speed: f32,
    pub map: WASDKeyMap,
    pub entity: Option<Entity>,
}

pub struct CtrlMove;

impl Default for CtrlConfig {
    fn default() -> Self {
        CtrlConfig {
            enabled: true,
            position: bevy::math::Vec3::new(0., 0.5, 0.),
            speed: 4.,
            map: WASDKeyMap::default(),
            entity: None,
        }
    }
}

fn use_ctrl(config: Res<CtrlConfig>) -> ShouldRun {
    if config.enabled {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn ctrl_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<CtrlConfig>,
) {
    if !config.enabled {
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
        .insert(CtrlMove);
}

pub fn validate_key<T>(codes: &'static [T], key: &T) -> bool
where
    T: PartialEq<T>,
{
    codes.iter().any(|m| m == key)
}

fn ctrl_update(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    config: Res<CtrlConfig>,
    mut transforms: Query<(&CtrlMove, &mut Transform)>,
) {
    for (_player, mut transform) in transforms.iter_mut() {
        let (_, mut rotation) = transform.rotation.to_axis_angle();
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = Vec3::new(local_z.x, 0., local_z.z);
        let right = transform.rotation * -Vec3::X;

        for key in keys.get_pressed() {
            if validate_key(config.map.forward, key) {
                velocity += forward
            }
            if validate_key(config.map.backward, key) {
                velocity -= forward
            }
            if validate_key(config.map.up, key) {
                velocity += Vec3::Y
            }
            if validate_key(config.map.down, key) {
                velocity -= Vec3::Y
            }
            if validate_key(config.map.left, key) {
                velocity -= right
            }
            if validate_key(config.map.right, key) {
                velocity += right
            }
            if validate_key(config.map.rot_left, key) {
                //Wrapping around
                if rotation > std::f32::consts::FRAC_PI_2 * 4.0 - 0.05 {
                    rotation = 0.0;
                }
                rotation += 0.1
            }
            if validate_key(config.map.rot_right, key) {
                //Wrapping around
                if rotation < 0.05 {
                    rotation = std::f32::consts::FRAC_PI_2 * 4.0;
                }
                rotation -= 0.1
            }
        }

        velocity = velocity.normalize();

        transform.rotation = Quat::from_rotation_y(rotation);

        if !velocity.is_nan() {
            transform.translation += velocity * time.delta_seconds() * config.speed;
        }
    }
}
