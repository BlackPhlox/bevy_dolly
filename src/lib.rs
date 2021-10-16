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

        // Update Position if needed
        if let Some(d) = rig.get_driver_mut::<Position>() {
            if d.init_set {
                d.position = transform.translation;
            }
        }

        // Update Rotation if needed
        if let Some(d) = rig.get_driver_mut::<Rotation>() {
            if d.init_set {
                d.rotation = transform.rotation;
            }
        }

        // if let Some(d) = rig.get_driver_mut::<YawPitch>() {
        //     if d.init_set {
        //         info!("transfrom: {:?}", transform.rotation);
        //         d.rotation = transform.rotation;
        //     }
        // }


        // for d in rig.drivers.iter() {
        //     info!("driver: {:?}", d);
        // }

        // Update once with no time
        //*transform = rig.update(0.0);
        // TODO: We could check for miss configurations here
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
            match transform_query.get(d.target_entity) {
                Ok(t) => d.target = t.clone(),
                Err(_) => (),
            }
        }
    }
}

fn apply_rigs_system(time: Res<Time>, mut query: Query<(&mut Transform, &mut Rig)>) {
    for (mut transform, mut rig) in query.iter_mut() {
        *transform = rig.update(time.delta_seconds());
    }
}
