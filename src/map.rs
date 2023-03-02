use bevy::prelude::*;
use dolly::prelude::RightHanded;
use std::marker::PhantomData;

pub type DollyTransformType = dolly::transform::Transform<RightHanded>;

#[derive(Deref, DerefMut)]
pub struct DollyTransform(dolly::transform::Transform<RightHanded>);

impl From<DollyTransformType> for DollyTransform {
    fn from(transform: DollyTransformType) -> Self {
        Self(transform)
    }
}

impl From<DollyTransform> for Transform {
    fn from(transform: DollyTransform) -> Self {
        let (translation, rotation) = transform.into_position_rotation();
        Self {
            translation,
            rotation,
            ..default()
        }
    }
}

impl From<Transform> for DollyTransform {
    fn from(transform: Transform) -> Self {
        Self(DollyTransformType {
            position: transform.translation,
            rotation: transform.rotation,
            phantom: PhantomData,
        })
    }
}

impl From<&Transform> for DollyTransform {
    fn from(transform: &Transform) -> Self {
        Self::from(*transform)
    }
}
impl From<Mut<'_, Transform>> for DollyTransform {
    fn from(transform: Mut<'_, Transform>) -> Self {
        Self::from(*transform.as_ref())
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
