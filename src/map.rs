use bevy::prelude::{default, Deref, DerefMut, Mut, Transform, Vec2};
use dolly::prelude::RightHanded;
use std::marker::PhantomData;

pub trait Transform2Bevy {
    fn transform_2_bevy(&mut self, transform: dolly::transform::Transform<RightHanded>);
}

pub trait Vec2Bevy {
    fn vector2d_2_bevy(&mut self, vec: dolly::glam::Vec2) -> Vec2;
}

impl Vec2Bevy for Vec2 {
    fn vector2d_2_bevy(&mut self, vec: dolly::glam::Vec2) -> Vec2 {
        Vec2 { x: vec.x, y: vec.y }
    }
}

pub trait Vec2Dolly {
    fn vector2d_2_dolly(&mut self) -> dolly::glam::Vec2;
}

impl Vec2Dolly for Vec2 {
    fn vector2d_2_dolly(&mut self) -> dolly::glam::Vec2 {
        dolly::glam::Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Transform2Bevy for Transform {
    fn transform_2_bevy(&mut self, transform: dolly::transform::Transform<RightHanded>) {
        let (translation, rotation) = transform.into_position_rotation();
        self.translation = bevy::math::Vec3::new(translation.x, translation.y, translation.z);
        self.rotation = bevy::math::Quat::from_xyzw(rotation.x, rotation.y, rotation.z, rotation.w);
    }
}

impl Transform2Bevy for Mut<'_, Transform> {
    fn transform_2_bevy(&mut self, transform: dolly::transform::Transform<RightHanded>) {
        let (translation, rotation) = transform.into_position_rotation();
        self.translation = bevy::math::Vec3::new(translation.x, translation.y, translation.z);
        self.rotation = bevy::math::Quat::from_xyzw(rotation.x, rotation.y, rotation.z, rotation.w);
    }
}

pub trait Transform2DollyMut {
    fn transform_2_dolly_mut(&self) -> dolly::transform::Transform<RightHanded>;
}

impl Transform2DollyMut for Mut<'_, Transform> {
    fn transform_2_dolly_mut(&self) -> dolly::transform::Transform<RightHanded> {
        let t = self.translation;
        let r = self.rotation;
        dolly::transform::Transform {
            position: dolly::glam::Vec3::new(t.x, t.y, t.z),
            rotation: dolly::glam::Quat::from_xyzw(r.x, r.y, r.z, r.w),
            phantom: PhantomData,
        }
    }
}

pub trait Transform2Dolly {
    fn transform_2_dolly(&self) -> dolly::transform::Transform<RightHanded>;
}

impl Transform2Dolly for Transform {
    fn transform_2_dolly(&self) -> dolly::transform::Transform<RightHanded> {
        let t = self.translation;
        let r = self.rotation;
        dolly::transform::Transform {
            position: dolly::glam::Vec3::new(t.x, t.y, t.z),
            rotation: dolly::glam::Quat::from_xyzw(r.x, r.y, r.z, r.w),
            phantom: PhantomData,
        }
    }
}

#[derive(Deref, DerefMut)]
struct DollyTransformInto(Transform);

impl From<dolly::transform::Transform<RightHanded>> for DollyTransformInto {
    fn from(transform: dolly::transform::Transform<RightHanded>) -> Self {
        let (translation, rotation) = transform.into_position_rotation();
        DollyTransformInto(Transform {
            translation: bevy::math::Vec3::new(translation.x, translation.y, translation.z),
            rotation: bevy::math::Quat::from_xyzw(rotation.x, rotation.y, rotation.z, rotation.w),
            ..default()
        })
    }
}
