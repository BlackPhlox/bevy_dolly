use std::marker::PhantomData;

use crate::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct DollyUpdateSet;

pub trait DollyComponent {
    fn add_dolly_component<T: Component>(&mut self, _: T) -> &mut Self;
    fn add_dolly_2d_component<T: Component>(&mut self, _: T) -> &mut Self;
    fn add_rig_component<T: Component>(&mut self, _: T) -> &mut Self;
}

impl DollyComponent for App {
    fn add_dolly_component<T: Component>(&mut self, _: T) -> &mut Self {
        self.add_systems(
            Update,
            Dolly::<T>::update_active_continuous.in_set(DollyUpdateSet),
        )
    }

    fn add_rig_component<T: Component>(&mut self, _: T) -> &mut Self {
        self.add_systems(
            Update,
            Dolly::<T>::update_all_continuous.in_set(DollyUpdateSet),
        )
    }

    fn add_dolly_2d_component<T: Component>(&mut self, _: T) -> &mut Self {
        self.add_systems(
            Update,
            Dolly::<T>::update_2d_active_continuous.in_set(DollyUpdateSet),
        )
    }
}

pub struct Dolly<T: Component> {
    _phantom: PhantomData<T>,
}

impl<T> Dolly<T>
where
    T: Component,
{
    #[allow(clippy::type_complexity)]
    pub fn update_active(
        mut cameras: Query<(&mut Transform, &Camera), With<T>>,
        time: Res<Time>,
        mut query: Query<&mut Rig, (Changed<Rig>, With<T>)>,
    ) {
        for mut rig in &mut query {
            //info!("{:?} changed: {:?}", entity, d);

            let transform = rig.update(time.delta_secs());

            cameras.iter_mut().for_each(|(mut t, camera)| {
                if camera.is_active {
                    *t = transform;
                }
            });
        }
    }

    const SCALE_INCR_THRESHOLD: f32 = 0.0025;
    const RANGE_SCALE_2D: f32 = 1. + Self::SCALE_INCR_THRESHOLD;

    pub fn update_2d_active(
        mut cameras: Query<(&mut Transform, &mut OrthographicProjection, &Camera), With<T>>,
        time: Res<Time>,
        mut query: Query<&mut Rig, (Changed<Rig>, With<T>)>,
    ) {
        for mut rig in &mut query {
            let mut transform = rig.update(time.delta_secs());
            cameras.iter_mut().for_each(|(mut t, mut orth, camera)| {
                if camera.is_active {
                    //Bind camera's Z axis to scale, if used for init state check to prevent scale of 0
                    if !(transform.translation.z < Self::RANGE_SCALE_2D
                        && transform.translation.z > -Self::RANGE_SCALE_2D)
                    {
                        orth.scale = (transform.translation.z + 1.) * Self::SCALE_INCR_THRESHOLD;
                    }
                    //Drop Z from camera's transform calculations and keep original
                    let xy = transform.translation.truncate().extend(t.translation.z);
                    transform.translation = xy;
                    *t = transform;
                }
            });
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn update_all(
        mut transforms: Query<&mut Transform, With<T>>,
        time: Res<Time>,
        mut query: Query<&mut Rig, (Changed<Rig>, With<T>)>,
    ) {
        for mut rig in &mut query {
            //info!("{:?} changed: {:?}", entity, d);

            let transform = rig.update(time.delta_secs());

            transforms.iter_mut().for_each(|mut t| {
                *t = transform;
            });
        }
    }

    // Continuous versions
    // Per default dolly only updates on input (change to rig)
    // This reduces update calls.
    // However, if camera movement is expected to last after input, ie. using large amount of motion smoothing < 0.25
    // Use the systems below instead.

    pub fn update_active_continuous(
        mut cameras: Query<(&mut Transform, &Camera), With<T>>,
        time: Res<Time>,
        mut query: Query<&mut Rig, With<T>>,
    ) {
        for mut rig in &mut query {
            //info!("{:?} changed: {:?}", entity, d);

            let transform = rig.update(time.delta_secs());

            cameras.iter_mut().for_each(|(mut t, camera)| {
                if camera.is_active {
                    *t = transform;
                }
            });
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn update_2d_active_continuous(
        mut cameras: Query<(&mut Transform, &mut OrthographicProjection, &Camera), With<T>>,
        time: Res<Time>,
        mut query: Query<&mut Rig, With<T>>,
    ) {
        for mut rig in &mut query {
            let mut transform = rig.update(time.delta_secs());
            cameras.iter_mut().for_each(|(mut t, mut orth, camera)| {
                if camera.is_active {
                    //Bind camera's Z axis to scale, if used for init state check to prevent scale of 0
                    if !(transform.translation.z < Self::RANGE_SCALE_2D
                        && transform.translation.z > -Self::RANGE_SCALE_2D)
                    {
                        orth.scale = (transform.translation.z + 1.) * Self::SCALE_INCR_THRESHOLD;
                    }
                    //Drop Z from camera's transform calculations and keep original
                    let xy = transform.translation.truncate().extend(t.translation.z);
                    transform.translation = xy;
                    *t = transform;
                }
            });
        }
    }

    pub fn update_all_continuous(
        mut transforms: Query<&mut Transform, With<T>>,
        time: Res<Time>,
        mut query: Query<&mut Rig, With<T>>,
    ) {
        for mut rig in &mut query {
            //info!("{:?} changed: {:?}", entity, d);

            let transform = rig.update(time.delta_secs());

            transforms.iter_mut().for_each(|mut t| {
                *t = transform;
            });
        }
    }
}
