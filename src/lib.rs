#![feature(derive_default_enum)]
pub mod bundle;
pub mod control;
pub mod rig;
use bevy::prelude::*;
use control::*;
use rig::*;

pub mod prelude {
    pub use crate::{bundle::*, control::*, rig::*, *};
}

pub struct DollyPlugin;
impl Plugin for DollyPlugin {
    fn build(&self, app: &mut App) {

        // TODO: We are getting a frame+1 lag, not worth fixing complexity with stages right now
        app.add_system_to_stage(CoreStage::PreUpdate, init_rig_system)
            .add_system_to_stage(CoreStage::PreUpdate, update_rigs_system)
            .add_system_to_stage(CoreStage::Update, apply_rigs_system)

            // These are only use for camera control system
            .init_resource::<DollyControlConfig>()
            .add_system_to_stage(CoreStage::PreUpdate, update_control_system);
    }
}

/// Listen for new Rigs
/// Add position and rotation info if needed
fn init_rig_system(mut query: Query<(&mut Transform, &mut Rig), Added<Rig>>) {
    for (transform, mut rig) in query.iter_mut() {
        // set init target for the transform
        rig.target = *transform;
    }
}

fn update_rigs_system(mut rig_query: Query<&mut Rig>, transform_query: Query<&Transform>) {
    for mut rig in rig_query.iter_mut() {
        // Update LookAt Drivers
        if let Some(d) = rig.get_driver_mut::<LookAt>() {
            if let Some(target_entity) = d.target_entity {
                if let Ok(target_transfrom) = transform_query.get(target_entity) {
                    d.target_transform = Some(*target_transfrom);
                }
            }
        }

        // Update Anchor Drivers
        if let Some(d) = rig.get_driver_mut::<Anchor>() {
            if let Ok(t) = transform_query.get(d.target_entity) {
                d.target = *t;
            }
        }
    }
}

fn apply_rigs_system(time: Res<Time>, mut query: Query<(&mut Transform, &mut Rig)>) {
    for (mut transform, mut rig) in query.iter_mut() {
        *transform = rig.update(time.delta_seconds());
    }
}
