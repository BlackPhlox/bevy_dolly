use crate::prelude::Rig;
use bevy::prelude::{
    App, Camera, Changed, Component, Entity, IntoSystemDescriptor, OrthographicProjection, Query,
    Res, SystemLabel, Time, Transform, With,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub struct DollyComponentLabel;

pub trait DollyComponent {
    fn add_dolly_component<T: Component>(&mut self, _: T) -> &mut Self;
    fn add_dolly_2d_component<T: Component>(&mut self, _: T) -> &mut Self;
    fn add_rig_component<T: Component>(&mut self, _: T) -> &mut Self;
}

impl DollyComponent for App {
    fn add_dolly_component<T: Component>(&mut self, _: T) -> &mut Self {
        self.add_system(dolly_component_cam_change_detection::<T>.label(DollyComponentLabel))
    }

    fn add_rig_component<T: Component>(&mut self, _: T) -> &mut Self {
        self.add_system(dolly_component_change_detection::<T>.label(DollyComponentLabel))
    }

    fn add_dolly_2d_component<T: Component>(&mut self, _: T) -> &mut Self {
        self.add_system(dolly_2d_component_cam_change_detection::<T>.label(DollyComponentLabel))
    }
}

#[allow(clippy::type_complexity)]
pub fn dolly_component_cam_change_detection<T: Component>(
    mut cameras: Query<(&mut Transform, &Camera), With<T>>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Rig), (Changed<Rig>, With<T>)>,
) {
    for (_entity, mut rig) in &mut query {
        //let d = rig.drivers.iter().map(|f| format!("{:?}", f)).collect::<Vec<String>>().join(", ");
        //info!("{:?} changed: {:?}", entity, d);

        let transform = rig.update(time.delta_seconds());

        cameras.for_each_mut(|(mut t, camera)| {
            if camera.is_active {
                *t = transform;
            }
        });
    }
}

#[allow(clippy::type_complexity)]
pub fn dolly_2d_component_cam_change_detection<T: Component>(
    mut cameras: Query<(&mut Transform, &mut OrthographicProjection, &Camera), With<T>>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Rig), (Changed<Rig>, With<T>)>,
) {
    for (_entity, mut rig) in &mut query {
        //let d = rig.drivers.iter().map(|f| format!("{:?}", f)).collect::<Vec<String>>().join(", ");
        //info!("{:?} changed: {:?}", entity, d);

        let mut transform = rig.update(time.delta_seconds());

        cameras.for_each_mut(|(mut t, mut orth, camera)| {
            if camera.is_active {
                orth.scale = transform.translation.z * 0.0025; //.clamp(0.0, 0.5);
                let xy = transform.translation.truncate().extend(0 as f32);
                transform.translation = xy;
                *t = transform;
            }
        });
    }
}

#[allow(clippy::type_complexity)]
pub fn dolly_component_change_detection<T: Component>(
    mut transforms: Query<&mut Transform, With<T>>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Rig), (Changed<Rig>, With<T>)>,
) {
    for (_entity, mut rig) in &mut query {
        //let d = rig.drivers.iter().map(|f| format!("{:?}", f)).collect::<Vec<String>>().join(", ");
        //info!("{:?} changed: {:?}", entity, d);

        let transform = rig.update(time.delta_seconds());

        transforms.for_each_mut(|mut t| {
            *t = transform;
        });
    }
}
