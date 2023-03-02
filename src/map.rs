use bevy::prelude::*;
use dolly::prelude::RightHanded;
use std::marker::PhantomData;

pub type DollyTransform = dolly::transform::Transform<RightHanded>;

#[derive(Deref, DerefMut)]
pub struct DollyTransformWrapper(dolly::transform::Transform<RightHanded>);

impl From<DollyTransform> for DollyTransformWrapper {
    fn from(transform: DollyTransform) -> Self {
        Self(transform)
    }
}

impl From<DollyTransformWrapper> for Transform {
    fn from(transform: DollyTransformWrapper) -> Self {
        let (translation, rotation) = transform.into_position_rotation();
        Self {
            translation,
            rotation,
            ..default()
        }
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
