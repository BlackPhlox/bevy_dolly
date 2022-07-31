use bevy::{prelude::{App, Camera, Changed, Entity, Query, Res, Time, Transform, Component, With, ParallelSystemDescriptorCoercion}, ecs::schedule::IntoSystemDescriptor};
use crate::prelude::{Transform2Bevy, Rig};

pub trait DollyComponent {
    fn add_dolly_component<T: Component>(&mut self, _: T) -> &mut Self;
}

impl DollyComponent for App {
    fn add_dolly_component<T: Component>(&mut self, _ : T) -> &mut Self {
        self.add_system(dolly_component_cam_change_detection::<T>)
    }
}

fn dolly_component_cam_change_detection<T: Component>(
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
                t.transform_2_bevy(transform);
            }
        });
    }
}
