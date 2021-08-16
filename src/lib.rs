use bevy::prelude::{AppBuilder, Mut, Plugin, Transform};
use ctrl::Ctrl;
use dolly::glam::{Quat, Vec3};

pub mod cone;
pub mod ctrl;

pub struct Dolly;
impl Plugin for Dolly {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(Ctrl);
    }
}

pub trait Transform2Bevy {
    fn transform2bevy(&mut self, transform: dolly::transform::Transform);
}

impl Transform2Bevy for Mut<'_, Transform> {
    fn transform2bevy(&mut self, transform: dolly::transform::Transform) {
        let (translation, rotation) = transform.into_translation_rotation();
        self.translation = bevy::math::Vec3::new(translation.x, translation.y, translation.z);
        self.rotation = bevy::math::Quat::from_xyzw(rotation.x, rotation.y, rotation.z, rotation.w);
    }
}

pub trait Transform2Dolly {
    fn transform2dolly(&self) -> dolly::transform::Transform;
}

impl Transform2Dolly for Mut<'_, Transform> {
    fn transform2dolly(&self) -> dolly::transform::Transform {
        let t = self.translation;
        let q = self.rotation;
        dolly::transform::Transform {
            translation: Vec3::new(t.x, t.y, t.z),
            rotation: Quat::from_xyzw(q.x, q.y, q.z, q.w),
        }
    }
}
