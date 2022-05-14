use std::marker::PhantomData;
use bevy::{prelude::{Mut, Transform}, math::{Vec3, Quat}};
use dolly::prelude::LeftHanded;

pub trait Transform2Bevy {
    fn transform_2_bevy(&mut self, transform: dolly::transform::Transform<LeftHanded>);
}

impl Transform2Bevy for Mut<'_, Transform> {
    fn transform_2_bevy(&mut self, transform: dolly::transform::Transform<LeftHanded>) {
        let (translation, rotation) = transform.into_position_rotation();
        self.translation = bevy::math::Vec3::new(translation.x, translation.y, translation.z);
        self.rotation = bevy::math::Quat::from_xyzw(rotation.x, rotation.y, rotation.z, rotation.w);
    }
}

pub trait Transform2DollyMut {
    fn transform_2_dolly_mut(&self) -> dolly::transform::Transform<LeftHanded>;
}

impl Transform2DollyMut for Mut<'_, Transform> {
    fn transform_2_dolly_mut(&self) -> dolly::transform::Transform<LeftHanded> {
        let t = self.translation;
        let r = self.rotation;
        dolly::transform::Transform {
            position: Vec3::new(t.x, t.y, t.z),
            rotation: Quat::from_xyzw(r.x, r.y, r.z, r.w),
            phantom: PhantomData
        }
    }
}

pub trait Transform2Dolly {
    fn transform_2_dolly(&self) -> dolly::transform::Transform<LeftHanded>;
}

impl Transform2Dolly for Transform {
    fn transform_2_dolly(&self) -> dolly::transform::Transform<LeftHanded> {
        let t = self.translation;
        let r = self.rotation;
        dolly::transform::Transform {
            position: Vec3::new(t.x, t.y, t.z),
            rotation: Quat::from_xyzw(r.x, r.y, r.z, r.w),
            phantom: PhantomData
        }
    }
}