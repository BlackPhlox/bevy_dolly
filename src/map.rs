use bevy::prelude::*;
use dolly::prelude::RightHanded;
use std::marker::PhantomData;

pub type DollyTransformType = dolly::transform::Transform<RightHanded>;

/// Utility wrapper to convert between bevy and dolly transforms.
/// Use it like this:
/// ```
/// use bevy::prelude::*;
/// use bevy_dolly::prelude::*;
///
/// // Get the transform from a Bevy query
/// let transform = Transform::default();
/// let dolly_transform = DollyTransform::from(transform);
/// ```
#[derive(Deref, DerefMut)]
pub struct DollyTransform(pub DollyTransformType);

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
