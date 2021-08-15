use bevy::prelude::{AppBuilder, Mut, Plugin, Transform};
use ctrl::Ctrl;

pub mod cone;
pub mod ctrl;

pub struct Dolly;
impl Plugin for Dolly {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(Ctrl);
    }
}

pub trait DollyCamUpdate {
    fn update(&mut self, t: dolly::transform::Transform);
}

impl DollyCamUpdate for Mut<'_, Transform> {
    fn update(&mut self, t: dolly::transform::Transform) {
        let (translation, rotation) = t.into_translation_rotation();
        self.translation = bevy::math::Vec3::new(translation.x, translation.y, translation.z);
        self.rotation = bevy::math::Quat::from_xyzw(rotation.x, rotation.y, rotation.z, rotation.w);
    }
}
